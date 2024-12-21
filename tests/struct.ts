import { deepStrictEqual } from "assert"
import {
  load,
  DataType,
  createPointer,
  restorePointer,
  unwrapPointer,
  arrayConstructor
} from "../index"
import { logGreen } from "./utils"
import { person, personType } from "./types"

export const testObject = () => {
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
    retType: arrayConstructor({
      type: DataType.StructArray,
      structItemType: {
        x: DataType.I16,
        y: DataType.I16,
        dir: DataType.I16,
        kind: DataType.U8,
        ffiTypeTag: DataType.StackStruct,
      },
      length: 3,
    }),
    paramsType: [
      arrayConstructor({
        type: DataType.StructArray,
        structItemType: {
          x: DataType.I16,
          y: DataType.I16,
          dir: DataType.I16,
          kind: DataType.U8,
        },
        length: 3,
      })
    ],
    paramsValue: [structArray],
  })
  deepStrictEqual(res, structArray)
  // load({
  //   library: "libsum",
  //   funcName: "printAndReturnMinutiae",
  //   retType: personType,
  //   paramsType: [
  //     {
  //       nNumber: DataType.I16,
  //       item: arrayConstructor({
  //         type: {
  //           x: DataType.I16,
  //           y: DataType.I16,
  //           dir: DataType.I16,
  //           kind: DataType.U8,
  //         },
  //         length: 2,
  //       }),
  //     }
  //   ],
  //   paramsValue: [{
  //     nNumber: 2,
  //     item: [
  //       {
  //         x: 1,
  //         y: 2,
  //         dir: 3,
  //         kind: 4,
  //       },
  //       {
  //         x: 2,
  //         y: 3,
  //         dir: 4,
  //         kind: 5,
  //       },
  //     ]
  //   }],
  //   freeResultMemory: false
  // })
  // const personObj = load({
  //   library: "libsum",
  //   funcName: "getStruct",
  //   retType: personType,
  //   paramsType: [
  //     personType
  //   ],
  //   paramsValue: [person],
  //   freeResultMemory: false
  // });
  // deepStrictEqual(person, personObj);
  // logGreen('test getStruct succeed')
  // const createdPerson = load({
  //   library: "libsum",
  //   funcName: "createPerson",
  //   retType: personType,
  //   paramsType: [],
  //   paramsValue: [],
  // });
  // deepStrictEqual(createdPerson, person);
  // logGreen('test createdPerson succeed')
  // let personPointer = createPointer({
  //   paramsType: [personType],
  //   paramsValue: [person]
  // })
  // const personObjByPointer = load({
  //   library: "libsum",
  //   funcName: "getStruct",
  //   retType: personType,
  //   paramsType: [
  //     DataType.External
  //   ],
  //   freeResultMemory: false,
  //   paramsValue: unwrapPointer(personPointer),
  // });
  // deepStrictEqual(person, personObjByPointer);
  // logGreen('test getStructByPointer succeed')
  // personPointer = createPointer({
  //   paramsType: [personType],
  //   paramsValue: [person]
  // })
  // const restorePersonObjByPointer = restorePointer({
  //   paramsValue: personPointer,
  //   retType: [personType]
  // })
  // deepStrictEqual(person, restorePersonObjByPointer[0])
}