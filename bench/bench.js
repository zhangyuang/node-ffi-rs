const b = require('benny')
const ffi = require('ffi-napi');
const koffi = require('koffi');
const { load, DataType, open } = require('../index')


const platform = process.platform
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"
const koffilib = koffi.load(dynamicLib);

open({
  library: 'libsum',
  path: dynamicLib
})
const libm = ffi.Library('./libsum', {
  'sum': ['int', ['int', 'int']],
  concatenateStrings: ['string', ['string', 'string']],
});
const koffi_libm = {
  sum: koffilib.func('int sum(int a, int b)'),
  concatenateStrings: koffilib.func('const char *concatenateStrings(const char *str1, const char *str2)')
};

async function run() {
  await b.suite(
    'ffi',
    b.add('ffi-napi', () => {
      libm.sum(1, 2);
      libm.concatenateStrings("foo", "bar");
    }),
    b.add('koffi', () => {
      koffi_libm.sum(1, 2)
      koffi_libm.concatenateStrings("foo", "bar")
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
    }),
    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
