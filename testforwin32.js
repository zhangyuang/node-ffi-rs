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

  console.log(load({
    library: "libsum",
    funcName: "TIMInit",
    retType: DataType.I32,
    paramsType: [DataType.I64, DataType.String],
    paramsValue: [1400187352, JSON.stringify({
      "sdk_config_log_file_path": "yuuangtest", "sdk_config_config_file_path": "yuuangtest"
    })],
  }))
  open({
    library: "libsum2",
    path: "./sum32.dll",
  });
  console.log(load({
    library: "libsum2",
    funcName: "concatenateStrings",
    retType: DataType.String,
    paramsType: [DataType.String],
    paramsValue: ["yuuang"],
  }))
  return

};

unitTest();



exports.unitTest = unitTest;
