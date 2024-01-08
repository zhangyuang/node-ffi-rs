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

const unitTest = () => {

  // console.log(load({
  //   library: "libsum",
  //   funcName: "TIMInit",
  //   retType: DataType.I32,
  //   paramsType: [DataType.I32, DataType.String],
  //   paramsValue: [1400187352, "{}"],
  // }))
  console.log(load({
    library: "libsum",
    funcName: "TIMInit",
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.String],
    paramsValue: [1400187352, JSON.stringify({
      a: 1
    })],
  }))

  return

};

unitTest();

unitTest();


exports.unitTest = unitTest;
