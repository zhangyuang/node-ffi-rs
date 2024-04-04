const { equal, deepStrictEqual } = require('assert')
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

  let bigArr = new Array(100).fill(100)
  equal(bigArr[0], load({
    library: dynamicLib,
    funcName: 'createArrayi32',
    retType: RetType.I32Array,
    paramsType: [ParamsType.I32Array, ParamsType.I32],
    paramsValue: [bigArr, bigArr.length],
    retTypeLen: bigArr.length
  })[0])

  let bigDoubleArr = new Array(100).fill(1.1)
  equal(bigDoubleArr[0], load({
    library: dynamicLib,
    funcName: 'createArrayDouble',
    retType: RetType.DoubleArray,
    paramsType: [ParamsType.DoubleArray, ParamsType.I32],
    paramsValue: [bigDoubleArr, bigDoubleArr.length],
    retTypeLen: bigDoubleArr.length
  })[0])

  let stringArr = [c, c.repeat(200)]
  equal(stringArr[0], load({
    library: dynamicLib,
    funcName: 'createArrayString',
    retType: RetType.StringArray,
    paramsType: [ParamsType.StringArray, ParamsType.I32],
    paramsValue: [stringArr, stringArr.length],
    retTypeLen: stringArr.length
  })[0])
  const bool_val = true
  equal(!bool_val, load({
    library: dynamicLib,
    funcName: 'return_opposite',
    retType: RetType.Boolean,
    paramsType: [ParamsType.Boolean],
    paramsValue: [bool_val],
  }))

  const person = {
    name: 'tom',
    age: 23,
  }
  const personObj = load({
    library: dynamicLib,
    funcName: 'getStruct',
    retType: RetType.Object,
    paramsType: [{
      name: ParamsType.String,
      age: ParamsType.I32,
    }],
    paramsValue: [person],
    retFields: {
      name: ParamsType.String,
      age: ParamsType.I32,
    }
  })
  equal(person.name, personObj.name)
  equal(person.age, personObj.age)

}

unitTest()

exports.unitTest = unitTest
