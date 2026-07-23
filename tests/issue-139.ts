import { equal, deepStrictEqual, ok } from "assert";
import {
  open,
  close,
  load,
  DataType,
  createPointer,
  restorePointer,
  unwrapPointer,
  wrapPointer,
  freePointer,
  createExternalBuffer,
  arrayConstructor,
  PointerType,
} from "../index";
import { logGreen } from "./utils";

// Access libc through the main-program handle (path: "").
// libc provides malloc/free/memset/memcpy, which is all we need to demonstrate
// zero-copy buffer wrapping without touching the bundled C++ test lib.
open({ library: "libc", path: "" });

const MALLOC = "malloc";
const FREE = "free";
const MEMSET = "memset";
const MEMCPY = "memcpy";

function ns(): number {
  const t = process.hrtime.bigint();
  return Number(t);
}

function alloc(size: number) {
  const ptr = load({
    library: "libc",
    funcName: MALLOC,
    retType: DataType.External,
    paramsType: [DataType.U64],
    paramsValue: [size],
  }) as unknown as { _externalDataPlaceholder: unknown };
  return ptr;
}

function freePtr(ptr: unknown) {
  load({
    library: "libc",
    funcName: FREE,
    retType: DataType.Void,
    paramsType: [DataType.External],
    paramsValue: [ptr],
  });
}

function memset(ptr: unknown, value: number, size: number) {
  load({
    library: "libc",
    funcName: MEMSET,
    retType: DataType.External,
    paramsType: [DataType.External, DataType.I32, DataType.U64],
    paramsValue: [ptr, value, size],
  });
}

function memcpy(dst: unknown, src: Buffer, size: number) {
  load({
    library: "libc",
    funcName: MEMCPY,
    retType: DataType.External,
    paramsType: [DataType.External, DataType.U8Array, DataType.U64],
    paramsValue: [dst, src, size],
  });
}

function testZeroCopyRead() {
  const size = 256;
  const ptr = alloc(size);
  ok(ptr, "malloc returned a non-null pointer");

  const known = Buffer.alloc(size, 0);
  for (let i = 0; i < size; i++) known[i] = (i * 7) & 0xff;
  memset(ptr, 0, size);
  memcpy(ptr, known, size);

  // Zero-copy wrap: no data is copied, the Buffer aliases the malloc'd region.
  const buf = createExternalBuffer(ptr, size);
  equal(buf.length, size, "buffer length matches requested size");
  deepStrictEqual(
    Buffer.from(buf),
    known,
    "zero-copy read: buffer contents equal the bytes at the pointer"
  );

  freePtr(ptr);
  logGreen("testZeroCopyRead passed");
}

function testSharedMutability() {
  const size = 64;
  const ptr = alloc(size);
  memset(ptr, 0, size);

  // Wrap the same memory once and mutate it through the Buffer.
  const bufA = createExternalBuffer(ptr, size);
  for (let i = 0; i < size; i++) bufA[i] = 0xa5 ^ (i & 0x1f);

  // Wrap the SAME pointer again. Because the first wrap is zero-copy, the two
  // Buffers must alias the same memory — a copy would have decoupled them.
  const bufB = createExternalBuffer(ptr, size);
  deepStrictEqual(
    Buffer.from(bufB),
    Buffer.from(bufA),
    "shared memory: writes through one Buffer are visible through another wrap of the same pointer"
  );

  // Mutating the second Buffer must also be visible via a third wrap.
  bufB[0] = 0x42;
  const bufC = createExternalBuffer(ptr, size);
  equal(bufC[0], 0x42, "shared memory: write through second wrap is visible to a third");

  freePtr(ptr);
  logGreen("testSharedMutability passed");
}

function testContrastWithRestorePointer() {
  // restorePointer COPIES: mutating its Buffer must NOT affect the pointer.
  const size = 32;
  const ptr = alloc(size);
  memset(ptr, 0x11, size);

  const ptrWrap = wrapPointer([ptr]);
  const restored = restorePointer({
    retType: [arrayConstructor({ type: DataType.U8Array, length: size })],
    paramsValue: ptrWrap,
  }) as unknown as Buffer[];

  equal(restored.length, 1);
  equal(Buffer.from(restored[0]).every((b) => b === 0x11), true, "restore reads 0x11 fill");

  // Mutate the copied buffer; the underlying memory must be untouched.
  for (let i = 0; i < size; i++) restored[0][i] = 0xee;

  const buf = createExternalBuffer(ptr, size);
  equal(
    Buffer.from(buf).every((b) => b === 0x11),
    true,
    "restorePointer copied: mutating its buffer does not affect the live pointer"
  );

  freePtr(ptr);
  logGreen("testContrastWithRestorePointer passed");
}

function testBenchmarkVsRestorePointer() {
  const size = 4096;
  const ptr = alloc(size);
  memset(ptr, 0x33, size);
  const ptrWrap = wrapPointer([ptr]);
  const desc = arrayConstructor({ type: DataType.U8Array, length: size });

  const iters = 50_000;
  // warm up
  for (let i = 0; i < 1000; i++) createExternalBuffer(ptr, size);
  for (let i = 0; i < 1000; i++)
    restorePointer({ retType: [desc], paramsValue: ptrWrap });

  const t0 = ns();
  for (let i = 0; i < iters; i++) createExternalBuffer(ptr, size);
  const wrapNs = ns() - t0;

  const t1 = ns();
  for (let i = 0; i < iters; i++)
    restorePointer({ retType: [desc], paramsValue: ptrWrap });
  const restoreNs = ns() - t1;

  const wrapAvg = wrapNs / iters;
  const restoreAvg = restoreNs / iters;

  console.log(
    `  createExternalBuffer avg: ${wrapAvg.toFixed(1)}ns/op | restorePointer avg: ${restoreAvg.toFixed(1)}ns/op | speedup ~${(restoreAvg / wrapAvg).toFixed(1)}x`
  );
  ok(
    wrapAvg < restoreAvg,
    "createExternalBuffer (zero-copy) is faster than restorePointer (copies)"
  );

  freePtr(ptr);
  logGreen("testBenchmarkVsRestorePointer passed");
}

function testNullPointerRejected() {
  // createPointer of a zero pointer yields a (null) external we can hand to
  // createExternalBuffer to confirm null pointers are rejected, not silently
  // wrapped.
  let threw = false;
  try {
    const nullExt = createPointer({
      paramsType: [DataType.External],
      paramsValue: [0],
    })[0];
    createExternalBuffer(nullExt, 16);
  } catch {
    threw = true;
  }
  ok(threw, "a null pointer must be rejected, not wrapped into a Buffer");
  logGreen("testNullPointerRejected passed");
}

async function main() {
  testZeroCopyRead();
  testSharedMutability();
  testContrastWithRestorePointer();
  testBenchmarkVsRestorePointer();
  testNullPointerRejected();
  close("libc");
  logGreen("issue-139 test passed");
}

main().catch((err) => {
  close("libc");
  console.error("issue-139 test FAILED:", err && err.stack ? err.stack : err);
  process.exitCode = 1;
});
