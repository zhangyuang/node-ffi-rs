import { equal, deepStrictEqual } from 'assert'
import { load, open, close, DataType, arrayConstructor, funcConstructor } from './index'

const platform = process.platform
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

open({
  library: 'libsum',
  path: dynamicLib
})

const unitTest = () => {
  const a = 1
  const b = 100
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
    doubleArray: [1.1, 2.2, 3.3],

    age: 23,
    doubleProps: 1.1,
    name: 'tom',
    stringArray: ["foo", "bar"],
    i32Array: [1, 2, 3, 4],
    testnum: 32,
    boolTrue: true,
    boolFalse: false
  }
  const personObj = load({
    library: 'libsum',
    funcName: 'getStruct',
    retType: {
      doubleArray: arrayConstructor({ type: DataType.DoubleArray, length: person.doubleArray.length }),
      age: DataType.I32,
      doubleProps: DataType.Double,
      name: DataType.String,
      stringArray: arrayConstructor({ type: DataType.StringArray, length: person.stringArray.length }),
      i32Array: arrayConstructor({ type: DataType.I32Array, length: person.i32Array.length }),
      testnum: DataType.I32,
      boolTrue: DataType.Boolean,
      boolFalse: DataType.Boolean,
    },
    paramsType: [{
      age: DataType.I32,
      doubleProps: DataType.Double,
      name: DataType.String,
      stringArray: DataType.StringArray,
      doubleArray: DataType.DoubleArray,
      i32Array: DataType.I32Array,
      testnum: DataType.I32,
      boolTrue: DataType.Boolean,
      boolFalse: DataType.Boolean,
    }],
    paramsValue: [person]
  })
  console.log('person', personObj)
  deepStrictEqual(person, personObj)
  const p = load({
    library: 'libsum',
    funcName: 'createPerson',
    retType: {
      doubleArray: arrayConstructor({ type: DataType.DoubleArray, length: 3 }),
      age: DataType.I32,
      doubleProps: DataType.Double,
      name: DataType.String,
      stringArray: arrayConstructor({ type: DataType.StringArray, length: 2 }),
      i32Array: arrayConstructor({ type: DataType.I32Array, length: 3 }),
      testnum: DataType.I32,
      boolTrue: DataType.Boolean,
      boolFalse: DataType.Boolean,
    },
    paramsType: [],
    paramsValue: []
  })
  console.log('createPerson', p)
  const newP = {
    doubleArray: [1, 2, 3],
    age: 30,
    doubleProps: 1.23,
    name: 'John Doe',
    stringArray: ['Hello', 'World'],
    i32Array: [1, 2, 3],
    testnum: 123,
    boolTrue: true,
    boolFalse: false
  }
  deepStrictEqual(p, newP)
  const func = (a, b, c, d, e, f) => {
    console.log('func params', a, b, c, d, e, f)
    equal(a, 100)
    equal(b, false)
    equal(c, 'Hello, World!')
    deepStrictEqual(d, ['Hello', 'world'])
    deepStrictEqual(e, [101, 202, 303])
    deepStrictEqual(f, newP)
  }

  load({
    library: 'libsum',
    funcName: 'callFunction',
    retType: DataType.Void,
    paramsType: [funcConstructor({
      paramsType: [DataType.I32, DataType.Boolean, DataType.String,
      arrayConstructor({ type: DataType.StringArray, length: 2 }),
      arrayConstructor({ type: DataType.I32Array, length: 3 }),
      {
        doubleArray: arrayConstructor({ type: DataType.DoubleArray, length: 3 }),
        age: DataType.I32,
        doubleProps: DataType.Double,
        name: DataType.String,
        stringArray: arrayConstructor({ type: DataType.StringArray, length: 2 }),
        i32Array: arrayConstructor({ type: DataType.I32Array, length: 3 }),
        testnum: DataType.I32,
        boolTrue: DataType.Boolean,
        boolFalse: DataType.Boolean,
      }
      ],
      retType: DataType.Void
    })],
    paramsValue: [func],
  })

  console.log('test succeed')
}

unitTest()

if (!process.env.MEMORY) {
  close('libsum')
}

exports.unitTest = unitTest
