import { equal, deepStrictEqual } from 'assert'
import { load, open, close, DataType, arrayConstructor } from './index'

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
  deepStrictEqual(bigArr, load({
    library: 'libsum',
    funcName: 'createArrayi32',
    retType: arrayConstructor({ type: DataType.I32Array, length: bigArr.length }),
    paramsType: [DataType.I32Array, DataType.I32],
    paramsValue: [bigArr, bigArr.length],
  }))

  let bigDoubleArr = new Array(5).fill(1.1)
  deepStrictEqual(bigDoubleArr, load({
    library: 'libsum',
    funcName: 'createArrayDouble',
    retType: arrayConstructor({ type: DataType.DoubleArray, length: bigDoubleArr.length }),
    paramsType: [DataType.DoubleArray, DataType.I32],
    paramsValue: [bigDoubleArr, bigDoubleArr.length],
  }))
  let stringArr = [c, c.repeat(20)]

  deepStrictEqual(stringArr, load({
    library: 'libsum',
    funcName: 'createArrayString',
    retType: arrayConstructor({ type: DataType.StringArray, length: stringArr.length }),
    paramsType: [DataType.StringArray, DataType.I32],
    paramsValue: [stringArr, stringArr.length],
  }))
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
    doubleProps: 1.1,
    stringArrProps: ["foo", "bar"]
  }
  const personObj = load({
    library: 'libsum',
    funcName: 'getStruct',
    retType: {
      name: DataType.String,
      age: DataType.I32,
      doubleProps: DataType.Double,
      stringArrProps: arrayConstructor({ type: DataType.StringArray, length: person.stringArrProps.length })
    },
    paramsType: [{
      name: DataType.String,
      age: DataType.I32,
      doubleProps: DataType.Double,
      stringArrProps: DataType.StringArray
    }],
    paramsValue: [person]
  })
  deepStrictEqual(person, personObj)
  // deepStrictEqual(person.stringArrProps, personObj.stringArrProps)
  // const func = () => {
  //   console.log('func')
  // }
  // load({
  //   library: 'libsum',
  //   funcName: 'callFunction',
  //   retType: DataType.Void,
  //   paramsType: [() => ({
  //     paramsType: [DataType.I32, DataType.String, DataType.Double],
  //     retType: DataType.Void
  //   })],
  //   paramsValue: [func],
  // })

}

unitTest()
close('libsum')

exports.unitTest = unitTest
