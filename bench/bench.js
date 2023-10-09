const b = require('benny')
const ffi = require('ffi-napi');
const { load, RetType, ParamsType } = require('../index')


const platform = process.platform
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

async function run() {
  await b.suite(
    'ffi',

    b.add('ffi-napi', () => {
      const libm = ffi.Library('libsum', {
        'sum': ['int', ['int', 'int']]
      });
      libm.sum(1, 2);
    }),

    b.add('ffi-rs', () => {
      load({
        library: dynamicLib,
        funcName: 'sum',
        retType: RetType.I32,
        paramsType: [ParamsType.I32, ParamsType.I32],
        paramsValue: [1, 2]
      })
    }),
    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
