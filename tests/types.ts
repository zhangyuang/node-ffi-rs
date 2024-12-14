import { DataType, arrayConstructor } from "../index"

export const parent = {
  age: 43,
  doubleArray: [1.1, 2.2, 3.3],
  parent: {},
  doubleProps: 3.3,
  name: "tom father",
  stringArray: ["tom", "father"],
  i32Array: [5, 6, 7],
  staticBytes: Buffer.from(new Array(16).fill(88)),
  boolTrue: true,
  boolFalse: false,
  longVal: 5294967296,
  stackStruct: {
    age: 22,
  },
  byte: 66,
  byteArray: Buffer.from([103, 104]),
};
export const person = {
  age: 23,
  doubleArray: [1.1, 2.2, 3.3],
  parent,
  doubleProps: 1.1,
  name: "tom",
  stringArray: ["tom"],
  i32Array: [1, 2, 3, 4],
  staticBytes: Buffer.from(new Array(16).fill(99)),
  boolTrue: true,
  boolFalse: false,
  longVal: 4294967296,
  stackStruct: {
    age: 16
  },
  byte: 65,
  byteArray: Buffer.from([101, 102]),
};
export const parentType = {
  age: DataType.I32,
  doubleArray: arrayConstructor({
    type: DataType.DoubleArray,
    length: parent.doubleArray.length,
  }),
  parent: {},
  doubleProps: DataType.Double,
  name: DataType.String,
  stringArray: arrayConstructor({
    type: DataType.StringArray,
    length: parent.stringArray.length,
  }),
  i32Array: arrayConstructor({
    type: DataType.I32Array,
    length: parent.i32Array.length,
  }),
  staticBytes: arrayConstructor({
    type: DataType.U8Array,
    length: parent.staticBytes.length,
    ffiTypeTag: DataType.StackArray
  }),
  boolTrue: DataType.Boolean,
  boolFalse: DataType.Boolean,
  longVal: DataType.I64,
  stackStruct: {
    age: DataType.I32,
    ffiTypeTag: DataType.StackStruct,
  },
  byte: DataType.U8,
  byteArray: arrayConstructor({
    type: DataType.U8Array,
    length: parent.byteArray.length,
  }),
};
export const personType = {
  age: DataType.I32,
  doubleArray: arrayConstructor({
    type: DataType.DoubleArray,
    length: person.doubleArray.length,
  }),
  parent: parentType,
  doubleProps: DataType.Double,
  name: DataType.String,
  stringArray: arrayConstructor({
    type: DataType.StringArray,
    length: person.stringArray.length,
  }),
  i32Array: arrayConstructor({
    type: DataType.I32Array,
    length: person.i32Array.length,
  }),
  staticBytes: arrayConstructor({
    type: DataType.U8Array,
    length: person.staticBytes.length,
    ffiTypeTag: DataType.StackArray
  }),
  boolTrue: DataType.Boolean,
  boolFalse: DataType.Boolean,
  longVal: DataType.I64,
  stackStruct: {
    ffiTypeTag: DataType.StackStruct,
    age: DataType.I32,
  },
  byte: DataType.U8,
  byteArray: arrayConstructor({
    type: DataType.U8Array,
    length: person.byteArray.length,
  }),
};