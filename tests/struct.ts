import { deepStrictEqual } from "assert"
import {
  load,
  DataType,
  createPointer,
  restorePointer,
  unwrapPointer,
  arrayConstructor,
  FFITypeTag
} from "../index"
import { logGreen } from "./utils"
import { person, personType } from "./types"

export const testObject = () => {
  const personObj = load({
    library: "libsum",
    funcName: "getStruct",
    retType: personType,
    paramsType: [
      personType
    ],
    paramsValue: [person],
    freeResultMemory: false
  });
  deepStrictEqual(person, personObj);
  logGreen('test getStruct succeed')
  const createdPerson = load({
    library: "libsum",
    funcName: "createPerson",
    retType: personType,
    paramsType: [],
    paramsValue: [],
  });
  deepStrictEqual(createdPerson, person);
  logGreen('test createdPerson succeed')
  let personPointer = createPointer({
    paramsType: [personType],
    paramsValue: [person]
  })
  const personObjByPointer = load({
    library: "libsum",
    funcName: "getStruct",
    retType: personType,
    paramsType: [
      DataType.External
    ],
    freeResultMemory: false,
    paramsValue: unwrapPointer(personPointer),
  });
  deepStrictEqual(person, personObjByPointer);
  logGreen('test getStructByPointer succeed')
  personPointer = createPointer({
    paramsType: [personType],
    paramsValue: [person]
  })
  const restorePersonObjByPointer = restorePointer({
    paramsValue: personPointer,
    retType: [personType]
  })
  deepStrictEqual(person, restorePersonObjByPointer[0])
  const structArray = [{
    x: 1,
    y: 2,
    dir: 3,
    kind: 4,
  },
  {
    x: 5,
    y: 6,
    dir: 7,
    kind: 8,
  },
  {
    x: 9,
    y: 10,
    dir: 11,
    kind: 12,
  }]
  const res = load({
    library: "libsum",
    funcName: "printAndReturnMinutiae",
    retType: {
      nNumber: DataType.I16,
      item: arrayConstructor({
        type: DataType.StructArray,
        ffiTypeTag: FFITypeTag.StackArray,
        structItemType: {
          x: DataType.I16,
          y: DataType.I16,
          dir: DataType.I16,
          kind: DataType.U8,
          ffiTypeTag: DataType.StackStruct
        },
        length: 3
      }),
    },
    paramsType: [{
      nNumber: DataType.I16,
      item: arrayConstructor({
        type: DataType.StructArray,
        ffiTypeTag: FFITypeTag.StackArray,
        structItemType: {
          x: DataType.I16,
          y: DataType.I16,
          dir: DataType.I16,
          kind: DataType.U8,
          ffiTypeTag: DataType.StackStruct
        },
        length: 3
      }),
    }],
    paramsValue: [{
      nNumber: 3,
      item: structArray,
    }]
  })
  deepStrictEqual(res, {
    nNumber: 3,
    item: structArray,
  })
}