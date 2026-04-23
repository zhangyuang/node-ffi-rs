import { open, load, close, DataType } from "../index";
import { logGreen } from "./utils";

// open main program handle to access libc functions
open({ library: "libc", path: "" });

async function testErrnoWithRunInNewThread() {
  // Sync path (control) — errno should work correctly
  const syncResult = load({
    library: "libc",
    funcName: "unlink",
    retType: DataType.I32,
    paramsType: [DataType.String],
    paramsValue: ["./__non_existent_file_issue135__"],
    errno: true,
  }) as { errnoCode: number; errnoMessage: string; value: number };

  console.log("sync result:", syncResult);
  if (syncResult.value !== -1) {
    throw new Error(`expected unlink to return -1, got ${syncResult.value}`);
  }
  if (syncResult.errnoCode === 0) {
    throw new Error("sync path: errnoCode should not be 0");
  }
  logGreen(`sync errno OK: code=${syncResult.errnoCode} msg="${syncResult.errnoMessage}"`);

  // Async path (runInNewThread) — this is the bug
  const asyncResult = (await load({
    library: "libc",
    funcName: "unlink",
    retType: DataType.I32,
    paramsType: [DataType.String],
    paramsValue: ["./__non_existent_file_issue135__"],
    runInNewThread: true,
    errno: true,
  })) as { errnoCode: number; errnoMessage: string; value: number };

  console.log("async result:", asyncResult);
  if (asyncResult.value !== -1) {
    throw new Error(`expected unlink to return -1, got ${asyncResult.value}`);
  }
  if (asyncResult.errnoCode === 0) {
    throw new Error(
      "BUG: runInNewThread + errno returns errnoCode=0 (errno not captured from worker thread)"
    );
  }
  logGreen(`async errno OK: code=${asyncResult.errnoCode} msg="${asyncResult.errnoMessage}"`);

  // Both should report the same errno (ENOENT = 2 on most platforms)
  if (syncResult.errnoCode !== asyncResult.errnoCode) {
    throw new Error(
      `errno mismatch: sync=${syncResult.errnoCode} async=${asyncResult.errnoCode}`
    );
  }
  logGreen("sync and async errno match");
}

testErrnoWithRunInNewThread()
  .then(() => {
    close("libc");
    logGreen("issue-135 test passed");
  })
  .catch((err) => {
    close("libc");
    console.error("issue-135 test FAILED:", err.message);
    process.exitCode = 1;
  });
