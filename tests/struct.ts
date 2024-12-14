import { deepStrictEqual } from "assert"
import {
  load,
  DataType,
  arrayConstructor,
  createPointer,
  restorePointer,
  unwrapPointer,
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
}