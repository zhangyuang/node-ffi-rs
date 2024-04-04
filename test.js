const { equal } = require('assert')
const { load, RetType, ParamsType } = require('./index')

const platform = process.platform
const a = 1
const b = 100

const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

equal(load({
  library: dynamicLib,
  funcName: 'sum',
  retType: RetType.I32,
  paramsType: [ParamsType.I32, ParamsType.I32],
  paramsValue: [a, b]
}), a + b)

const c = "foo"
const d = "bar"


equal(c + d, load({
  library: dynamicLib,
  funcName: 'concatenateStrings',
  retType: ParamsType.String,
  paramsType: [ParamsType.String, ParamsType.String],
  paramsValue: [c, d]
}))




// "optionalDependencies": {
//   "@yuuang/ffi-rs-win32-x64-msvc": "1.0.3",
//     "@yuuang/ffi-rs-darwin-x64": "1.0.3",
//       "@yuuang/ffi-rs-linux-x64-gnu": "1.0.3",
//         "@yuuang/ffi-rs-darwin-arm64": "1.0.3",
//           "@yuuang/ffi-rs-linux-arm64-gnu": "1.0.3",
//             "@yuuang/ffi-rs-linux-arm64-musl": "1.0.3"
// }
