import {
  open,
  close,
  createPointer,
  freePointer,
  DataType,
  PointerType,
  arrayConstructor,
} from "../index";
import { logGreen } from "./utils";

open({ library: "libc", path: "" });

function measureRss(): number {
  if (typeof process.memoryUsage === "function") {
    return process.memoryUsage().rss;
  }
  return 0;
}

function testU8ArrayFreePointer(iterations: number) {
  const buf = Buffer.alloc(1024, 0x42);
  const desc = arrayConstructor({
    type: DataType.U8Array,
    length: buf.length,
  });
  for (let i = 0; i < iterations; i++) {
    const ptrs = createPointer({
      paramsType: [desc],
      paramsValue: [buf],
    });
    freePointer({
      paramsType: [desc],
      paramsValue: ptrs,
      pointerType: PointerType.RsPointer,
    });
  }
}

function testI32ArrayFreePointer(iterations: number) {
  const arr = new Array(256).fill(42);
  const desc = arrayConstructor({
    type: DataType.I32Array,
    length: arr.length,
  });
  for (let i = 0; i < iterations; i++) {
    const ptrs = createPointer({
      paramsType: [desc],
      paramsValue: [arr],
    });
    freePointer({
      paramsType: [desc],
      paramsValue: ptrs,
      pointerType: PointerType.RsPointer,
    });
  }
}

const ITERATIONS = 50_000;
const THRESHOLD_MB = 10;

// Warm up
testU8ArrayFreePointer(100);
testI32ArrayFreePointer(100);

if (global.gc) global.gc();
const baseRss = measureRss();

testU8ArrayFreePointer(ITERATIONS);
if (global.gc) global.gc();
const afterU8 = measureRss();
const u8GrowthMb = (afterU8 - baseRss) / (1024 * 1024);

testI32ArrayFreePointer(ITERATIONS);
if (global.gc) global.gc();
const afterI32 = measureRss();
const i32GrowthMb = (afterI32 - afterU8) / (1024 * 1024);

logGreen(`U8Array: RSS grew ${u8GrowthMb.toFixed(1)} MB over ${ITERATIONS} iterations`);
logGreen(`I32Array (control): RSS grew ${i32GrowthMb.toFixed(1)} MB over ${ITERATIONS} iterations`);

const leakDelta = u8GrowthMb - i32GrowthMb;
if (leakDelta > THRESHOLD_MB) {
  console.error(
    `LEAK: U8Array grew ${leakDelta.toFixed(1)} MB more than I32Array control (threshold: ${THRESHOLD_MB} MB)`
  );
  process.exitCode = 1;
} else {
  logGreen("issue-134 U8Array test passed — no significant leak detected");
}

close("libc");
