const { equal, deepStrictEqual } = require('assert')
const { load, ParamsType, open, close, DataType } = require('./index')

const platform = process.platform
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

open({
  library: 'libsum',
  path: dynamicLib
})

const unitTest = () => {
  equal(load({
    library: 'libsum',
    funcName: 'sum',
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.I32],
    paramsValue: [a, b]
  }), a + b)

  const c = "foo"
  const d = c.repeat(200)

  equal(c + d, load({
    library: 'libsum',
    funcName: 'concatenateStrings',
    retType: DataType.String,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d]
  }))

  equal(undefined, load({
    library: 'libsum',
    funcName: 'noRet',
    retType: DataType.Void,
    paramsType: [],
    paramsValue: []
  }))


  equal(1.1 + 2.2, load({
    library: 'libsum',
    funcName: 'doubleSum',
    retType: DataType.Double,
    paramsType: [DataType.Double, DataType.Double],
    paramsValue: [1.1, 2.2]
  }))

  let bigArr = new Array(100).fill(100)
  equal(bigArr[0], load({
    library: 'libsum',
    funcName: 'createArrayi32',
    retType: DataType.I32Array,
    paramsType: [DataType.I32Array, DataType.I32],
    paramsValue: [bigArr, bigArr.length],
    retTypeLen: bigArr.length
  })[0])

  let bigDoubleArr = new Array(100).fill(1.1)
  equal(bigDoubleArr[0], load({
    library: 'libsum',
    funcName: 'createArrayDouble',
    retType: DataType.DoubleArray,
    paramsType: [DataType.DoubleArray, DataType.I32],
    paramsValue: [bigDoubleArr, bigDoubleArr.length],
    retTypeLen: bigDoubleArr.length
  })[0])

  let stringArr = [c, c.repeat(200)]
  equal(stringArr[0], load({
    library: 'libsum',
    funcName: 'createArrayString',
    retType: DataType.StringArray,
    paramsType: [DataType.StringArray, DataType.I32],
    paramsValue: [stringArr, stringArr.length],
    retTypeLen: stringArr.length
  })[0])
  const bool_val = true
  equal(!bool_val, load({
    library: 'libsum',
    funcName: 'return_opposite',
    retType: DataType.Boolean,
    paramsType: [DataType.Boolean],
    paramsValue: [bool_val],
  }))

  const person = {
    name: 'tom',
    age: 23,
  }
  const personObj = load({
    library: 'libsum',
    funcName: 'getStruct',
    retType: {
      name: DataType.String,
      age: DataType.I32,
    },
    paramsType: [{
      name: DataType.String,
      age: DataType.I32,
    }],
    paramsValue: [person]
  })
  equal(person.name, personObj.name)
  equal(person.age, personObj.age)
  const func = () => {
    console.log('func')
  }
  load({
    library: 'libsum',
    funcName: 'callFunction',
    retType: DataType.Void,
    paramsType: [DataType.Function],
    paramsValue: [func],
  })
}

unitTest()
close('libsum')

exports.unitTest = unitTest
