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

  console.log(load({
    library: "libsum",
    funcName: "TIMInit",
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.String],
    paramsValue: [1400187352, "{}"],
  }))
  console.log(load({
    library: "libsum",
    funcName: "TIMInit",
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.String],
    paramsValue: [1400187352, JSON.stringify({})],
  }))

  return
  const TIMAddRecvNewMsgCallback = (...args) => {
    console.log("TIMAddRecvNewMsgCallback", args);
  };
  load({
    library: "libsum",
    funcName: "TIMAddRecvNewMsgCallback",
    retType: DataType.Void,
    paramsType: [
      funcConstructor({
        paramsType: [DataType.String, DataType.String],
        permanent: true,
      }),
      DataType.String,
    ],
    paramsValue: [TIMAddRecvNewMsgCallback, JSON.stringify({})],
  });
  const bar = load({
    library: "libsum",
    funcName: "TIMLogin",
    retType: DataType.I32,
    paramsType: [
      DataType.String,
      DataType.String,
      funcConstructor({
        paramsType: [
          DataType.I32,
          DataType.String,
          DataType.String,
          DataType.String,
        ],
        retType: DataType.Void,
      }),
      DataType.String,
    ],
    paramsValue: [
      "109442",
      "eJyrVgrxCdYrSy1SslIy0jNQ0gHzM1NS80oy0zLBwoYGliYmRlCZ4pTsxIKCzBQlK0MTAwNDC3NjUyOITGpFQWZRKlDc1NTUyMDAACJakpkLEjOzNDM3sTA0soCakpkONFjbKLHQrawqNy8nyKkwoLzS0CUq2ygvsyDDJEbfw1E7JDm0yNItOzUwPdMx3VapFgAFFDFV",
      function(...args) {
        console.log("logincallback", args);
        const arr = Buffer.alloc(200)
        const res = load({
          library: "libsum",
          funcName: "TIMGetLoginUserID",
          retType: DataType.I32,
          paramsType: [
            DataType.U8Array
          ],
          paramsValue: [arr]
        })
        console.log('TIMGetLoginUserID', arr.toString())
        deepEqual(arr.toString().startsWith('109442'), true)
        deepEqual(res, 0)
      },
      JSON.stringify({}),
    ],
  });
  console.log("TIMLoginbar", bar);
};

unitTest();

unitTest();


exports.unitTest = unitTest;
