const b = require('benny')
const ffi = require('ffi-napi');
const { load, DataType, open } = require('../index')


const platform = process.platform
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

open({
  library: 'libsum',
  path: dynamicLib
})
open({
  library: "libnative",
  path: "",
});

const libm = ffi.Library('./libsum', {
  'sum': ['int', ['int', 'int']],
  concatenateStrings: ['string', ['string', 'string']],
});

const current = ffi.Library(null, {
  'atoi': ['int', ['string']]
});

async function run() {
  await b.suite(
    'ffi',
    b.add('ffi-napi', () => {
      libm.sum(1, 2);
      libm.concatenateStrings("foo", "bar");
      current.atoi("1000")
    }),
    b.add('ffi-rs', () => {
      load({
        library: 'libsum',
        funcName: 'sum',
        retType: DataType.I32,
        paramsType: [DataType.I32, DataType.I32],
        paramsValue: [1, 2]
      })
      load({
        library: 'libsum',
        funcName: 'concatenateStrings',
        retType: DataType.String,
        paramsType: [DataType.String, DataType.String],
        paramsValue: ["foo", "bar"]
      })
      load({
        library: "libnative",
        funcName: "atoi",
        retType: DataType.I32,
        paramsType: [DataType.String],
        paramsValue: ["1000"],
      })
    }),
    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
