import {
  createPointer,
  freePointer,
  arrayConstructor,
  DataType,
  PointerType,
} from "../index";

const ITERATIONS = 100_000;
const BUFFER_SIZE = 1024;
const LEAK_THRESHOLD_MB = 5;

function getRssMB(): number {
  if (global.gc) global.gc();
  return +(process.memoryUsage().rss / 1024 / 1024).toFixed(1);
}

function testU8ArrayLeak() {
  console.log("=== Bug 1: U8Array freePointer leak ===");

  // Warmup
  for (let i = 0; i < 1000; i++) {
    const ext = createPointer({
      paramsType: [arrayConstructor({ type: DataType.U8Array, length: BUFFER_SIZE })],
      paramsValue: [Buffer.alloc(BUFFER_SIZE)],
    });
    freePointer({
      paramsType: [arrayConstructor({ type: DataType.U8Array, length: BUFFER_SIZE })],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }

  const rssStart = getRssMB();
  console.log(`start: RSS = ${rssStart} MB`);

  for (let i = 0; i < ITERATIONS; i++) {
    const ext = createPointer({
      paramsType: [arrayConstructor({ type: DataType.U8Array, length: BUFFER_SIZE })],
      paramsValue: [Buffer.alloc(BUFFER_SIZE)],
    });
    freePointer({
      paramsType: [arrayConstructor({ type: DataType.U8Array, length: BUFFER_SIZE })],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }

  const rssEnd = getRssMB();
  const growth = +(rssEnd - rssStart).toFixed(1);
  console.log(`end:   RSS = ${rssEnd} MB (growth: +${growth} MB)`);

  // Compare with I32Array baseline
  const i32Start = getRssMB();
  for (let i = 0; i < ITERATIONS; i++) {
    const ext = createPointer({
      paramsType: [arrayConstructor({ type: DataType.I32Array, length: 4 })],
      paramsValue: [[1, 2, 3, 4]],
    });
    freePointer({
      paramsType: [arrayConstructor({ type: DataType.I32Array, length: 4 })],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }
  const i32Growth = +(getRssMB() - i32Start).toFixed(1);
  console.log(`I32Array baseline growth: +${i32Growth} MB`);

  const excessGrowth = +(growth - i32Growth).toFixed(1);
  if (excessGrowth > LEAK_THRESHOLD_MB) {
    console.log(`LEAK CONFIRMED: U8Array excess growth over I32Array baseline: +${excessGrowth} MB`);
    process.exitCode = 1;
  } else {
    console.log(`PASS: U8Array growth within baseline (excess: ${excessGrowth} MB)`);
  }
  console.log();
}

function testStackStructLeak() {
  console.log("=== Bug 2: StackStruct freePointer leak ===");
  const structType = {
    ffiTypeTag: DataType.StackStruct,
    code: DataType.I32,
    value: DataType.I64,
  };

  // Warmup
  for (let i = 0; i < 1000; i++) {
    const ext = createPointer({
      paramsType: [structType],
      paramsValue: [{ code: 0, value: 0 }],
    });
    freePointer({
      paramsType: [structType],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }

  const rssStart = getRssMB();
  console.log(`start: RSS = ${rssStart} MB`);

  for (let i = 0; i < ITERATIONS; i++) {
    const ext = createPointer({
      paramsType: [structType],
      paramsValue: [{ code: 0, value: 0 }],
    });
    freePointer({
      paramsType: [structType],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }

  const rssEnd = getRssMB();
  const growth = +(rssEnd - rssStart).toFixed(1);
  console.log(`end:   RSS = ${rssEnd} MB (growth: +${growth} MB)`);

  // Compare with I32 scalar baseline (minimal native alloc)
  const baseStart = getRssMB();
  for (let i = 0; i < ITERATIONS; i++) {
    const ext = createPointer({
      paramsType: [DataType.I32],
      paramsValue: [42],
    });
    freePointer({
      paramsType: [DataType.I32],
      paramsValue: ext,
      pointerType: PointerType.RsPointer,
    });
  }
  const baseGrowth = +(getRssMB() - baseStart).toFixed(1);
  console.log(`I32 scalar baseline growth: +${baseGrowth} MB`);

  const excessGrowth = +(growth - baseGrowth).toFixed(1);
  if (excessGrowth > LEAK_THRESHOLD_MB) {
    console.log(`LEAK CONFIRMED: StackStruct excess growth over baseline: +${excessGrowth} MB`);
    process.exitCode = 1;
  } else {
    console.log(`PASS: StackStruct growth within baseline (excess: ${excessGrowth} MB)`);
  }
  console.log();
}

testU8ArrayLeak();
testStackStructLeak();
