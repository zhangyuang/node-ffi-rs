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

open({
  library: "libsum",
  path: dynamicLib,
});

const unitTest = () => {
  const foo = load({
    library: "libsum",
    funcName: "TIMSetLogCallback",
    retType: DataType.I32,
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
  console.log('TIMSetLogCallback', foo)
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
