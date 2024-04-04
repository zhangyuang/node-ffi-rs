const { equal } = require('assert')
const p = require('./index')

const a = 1
const b = -100

equal(p.load({
  library: "./libsum.so",
  funcName: 'sum',
  retType: 1,
  paramsType: [1, 1],
  paramsValue: [a, b]
}), a + b)

const c = "foo"
const d = "bar"

equal(c + b, p.load({
  library: "./libsum.so",
  funcName: 'concatenateStrings',
  retType: 0,
  paramsType: [0, 0],
  paramsValue: ["a", "b"]
}))




// "optionalDependencies": {
//   "ffi-rs-win32-x64-msvc": "1.0.3",
//   "ffi-rs-darwin-x64": "1.0.3",
//   "ffi-rs-linux-x64-gnu": "1.0.3",
//   "ffi-rs-darwin-arm64": "1.0.3",
//   "ffi-rs-linux-arm64-gnu": "1.0.3",
//   "ffi-rs-linux-arm64-musl": "1.0.3"
// }
