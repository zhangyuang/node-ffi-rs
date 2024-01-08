const { equal, deepStrictEqual } = require("assert");
const {
  arrayConstructor,
  funcConstructor,
} = require("./");
const {
  load,
  open,
  close,
  DataType,
} = require("./")
const dynamicLib = "./ImSDK.dll"

console.log(process.arch, process.platform)
console.log(require("."))
open({
  library: "libsum",
  path: dynamicLib,
});
setInterval(() => {
  console.log('running')
}, 1000)
const unitTest = () => {
  load({
    library: "libsum",
    funcName: "TIMSetLogCallback",
    retType: DataType.Void,
    paramsType: [
      funcConstructor({
        paramsType: [
          DataType.I32,
          DataType.String,
          DataType.String,
        ],
        permanent: true,
      }),
      DataType.String,
    ],
    paramsValue: [(...args) => {
      console.log('xxx', args);
    }, ""],
  });
  console.log(load({
    library: "libsum",
    funcName: "TIMInit",
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.String],
    paramsValue: [1400187352, JSON.stringify({
      "sdk_config_log_file_path": "yuuangtest", "sdk_config_config_file_path": "yuuangtest"
    })],
  }))

  return

};

unitTest();



exports.unitTest = unitTest;
