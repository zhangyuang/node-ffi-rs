const { equal } = require('assert')
const { load, RetType, ParamsType } = require('./index')

const platform = process.platform
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

const unitTest = () => {
  equal(load({
    library: dynamicLib,
    funcName: 'sum',
    retType: RetType.I32,
    paramsType: [ParamsType.I32, ParamsType.I32],
    paramsValue: [a, b]
  }), a + b)

  const c = "foo"
  const d = c.repeat(200)

  equal(c + d, load({
    library: dynamicLib,
    funcName: 'concatenateStrings',
    retType: RetType.String,
    paramsType: [ParamsType.String, ParamsType.String],
    paramsValue: [c, d]
  }))

  equal(undefined, load({
    library: dynamicLib,
    funcName: 'noRet',
    retType: RetType.Void,
    paramsType: [],
    paramsValue: []
  }))


  equal(1.1 + 2.2, load({
    library: dynamicLib,
    funcName: 'doubleSum',
    retType: RetType.Double,
    paramsType: [ParamsType.Double, ParamsType.Double],
    paramsValue: [1.1, 2.2]
  }))

  let bigArr = new Array(100000).fill(100)
  equal(Math.max(bigArr), Math.max(load({
    library: dynamicLib,
    funcName: 'createArrayi32',
    retType: RetType.I32Array,
    paramsType: [ParamsType.I32Array, ParamsType.I32],
    paramsValue: [bigArr, bigArr.length],
    retTypeLen: bigArr.length
  })))

}

unitTest()

exports.unitTest = unitTest
