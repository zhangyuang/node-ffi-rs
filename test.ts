import { equal, deepStrictEqual } from "assert"
import {
  load,
  open,
  close,
  DataType,
  arrayConstructor,
  funcConstructor,
  createPointer,
  restorePointer,
  unwrapPointer,
  wrapPointer,
  freePointer,
  define,
  PointerType
} from "./index"


const platform = process.platform;
const dynamicLib = platform === "win32" ? "./sum.dll" : "./libsum.so";
const logGreen = (text) => {
  if (process.env.SILENT) return
  console.log('\x1b[32m%s\x1b[0m', text);
}

open({
  library: "libsum",
  path: dynamicLib,
});

open({
  library: "libnative",
  path: "",
});


const testNumber = () => {
  const a = 1;
  const b = 100;
  equal(
    load({
      library: "libsum",
      funcName: "sum",
      retType: DataType.I32,
      paramsType: [DataType.I32, DataType.I32],
      paramsValue: [a, b],
    }),
    a + b,
  );
  equal(
    1.1 + 2.2,
    load({
      library: "libsum",
      funcName: "doubleSum",
      retType: DataType.Double,
      paramsType: [DataType.Double, DataType.Double],
      paramsValue: [1.1, 2.2],
    }),
  );
  equal(
    1.5 + 2.5,
    load({
      library: "libsum",
      funcName: "floatSum",
      retType: DataType.Double,
      paramsType: [DataType.Float, DataType.Float],
      paramsValue: [1.5, 2.5],
    }),
  );
}
const c = "foo".repeat(100);
const d = "bar"
const testString = () => {
  equal(
    c + d,
    load({
      library: "libsum",
      funcName: "concatenateStrings",
      retType: DataType.String,
      paramsType: [DataType.String, DataType.String],
      paramsValue: [c, d],
    }),
  );
  equal(
    c + d,
    load({
      library: "libsum",
      funcName: "concatenateWideStrings",
      retType: DataType.WString,
      paramsType: [DataType.WString, DataType.WString],
      paramsValue: [c, d],
    }),
  );
}
const testVoid = () => {
  equal(
    undefined,
    load({
      library: "libsum",
      funcName: "noRet",
      retType: DataType.Void,
      paramsType: [],
      paramsValue: [],
    }),
  );
}
const testBool = () => {
  const bool_val = true;
  equal(
    !bool_val,
    load({
      library: "libsum",
      funcName: "return_opposite",
      retType: DataType.Boolean,
      paramsType: [DataType.Boolean],
      paramsValue: [bool_val],
    }),
  );
}
const testArray = () => {
  let stringArr = [c, c.repeat(200)];
  deepStrictEqual(
    stringArr,
    load({
      library: "libsum",
      funcName: "createArrayString",
      retType: arrayConstructor({
        type: DataType.StringArray,
        length: stringArr.length,
      }),
      paramsType: [arrayConstructor({
        type: DataType.StringArray,
        length: stringArr.length,
      }), DataType.I32],
      paramsValue: [stringArr, stringArr.length],
    }),
  );
  logGreen('test createArrayString succeed')
  let bigArr = new Array(100).fill(100);
  deepStrictEqual(
    bigArr,
    load({
      library: "libsum",
      funcName: "createArrayi32",
      retType: arrayConstructor({
        type: DataType.I32Array,
        length: bigArr.length,
      }),
      paramsType: [arrayConstructor({
        type: DataType.I32Array,
        length: bigArr.length,
      }), DataType.I32],
      paramsValue: [bigArr, bigArr.length],
    }),
  );

  let bigDoubleArr = new Array(5).fill(1.1);
  deepStrictEqual(
    bigDoubleArr,
    load({
      library: "libsum",
      funcName: "createArrayDouble",
      retType: arrayConstructor({
        type: DataType.DoubleArray,
        length: bigDoubleArr.length,
      }),
      paramsType: [arrayConstructor({
        type: DataType.DoubleArray,
        length: bigDoubleArr.length,
      }), DataType.I32],
      paramsValue: [bigDoubleArr, bigDoubleArr.length],
    }),
  );
}
const testPointer = () => {
  const i32Ptr = createPointer({
    paramsType: [DataType.I32],
    paramsValue: [100]
  })
  const i32Data = restorePointer({
    retType: [DataType.I32],
    paramsValue: i32Ptr
  })
  freePointer({
    paramsType: [DataType.I32],
    paramsValue: i32Ptr,
    pointerType: PointerType.RsPointer
  })
  deepStrictEqual(i32Data[0], 100)
  logGreen('test create and restore i32 pointer success')
  const stringPointer = createPointer({
    paramsType: [DataType.String],
    paramsValue: ["foo"]
  })
  const stringData = restorePointer({
    retType: [DataType.String],
    paramsValue: stringPointer
  })
  freePointer({
    paramsType: [DataType.String],
    paramsValue: stringPointer,
    pointerType: PointerType.RsPointer
  })
  logGreen('test create string pointer success')
  equal(load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: unwrapPointer(createPointer({
      paramsType: [DataType.String],
      paramsValue: ["foo"]
    })),
  }), "foo")
  const ptr = load({
    library: "libsum",
    funcName: "concatenateStrings",
    retType: DataType.External,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d],
  })
  const string = load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: [ptr],
  })
  equal(string, c + d)
  deepStrictEqual(stringData[0], "foo")
  logGreen('test string pointer success')
  const restoreData = restorePointer({
    retType: [arrayConstructor({
      type: DataType.DoubleArray,
      length: 2
    })],
    paramsValue: createPointer({
      paramsType: [DataType.DoubleArray],
      paramsValue: [[1.1, 2.2]]
    })
  })
  deepStrictEqual(restoreData, [[1.1, 2.2]])
  const ptrToI32Ptr = wrapPointer(createPointer({
    paramsType: [DataType.I32],
    paramsValue: [100]
  }));
  const getValueFromDoublePointer = load({
    library: "libsum",
    funcName: "getValueFromDoublePointer",
    retType: DataType.I32,
    paramsType: [DataType.External],
    paramsValue: ptrToI32Ptr
  })
  equal(getValueFromDoublePointer, 100)
  logGreen('test getValueFromDoublePointer success')
}
const parent = {
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
const person = {
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
const parentType = {
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
    dynamicArray: false
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
const personType = {
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
    dynamicArray: false
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
const testObject = () => {
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

const testRunInNewThread = () => {
  load({
    library: "libsum",
    funcName: "sum",
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.I32],
    paramsValue: [1, 2],
    runInNewThread: true,
  }).then(res => {
    equal(res, 3)
    logGreen('test runInNewThread succeed')
  })
}

const testFunction = () => {
  const func = (a, b, c, d, e, f, g) => {
    equal(a, 100);
    equal(b, false);
    equal(c, "Hello, World!");
    equal(d, "100.11");
    deepStrictEqual(e, ["Hello", "world"]);
    deepStrictEqual(f, [101, 202, 303]);
    deepStrictEqual(g, person);
    logGreen("test function succeed");
    // free function memory which malloc in c side when it not in use
    freePointer({
      paramsType: [funcConstructor({
        paramsType: [
          DataType.I32,
          DataType.Boolean,
          DataType.String,
          DataType.Double,
          arrayConstructor({ type: DataType.StringArray, length: 2 }),
          arrayConstructor({ type: DataType.I32Array, length: 3 }),
          personType,
        ],
        retType: DataType.Void,
      })],
      paramsValue: funcExternal,
      pointerType: PointerType.RsPointer
    })
    if (!process.env.MEMORY) {
      close("libsum");
    }
  };
  const funcExternal = createPointer({
    paramsType: [funcConstructor({
      paramsType: [
        DataType.I32,
        DataType.Boolean,
        DataType.String,
        DataType.Double,
        arrayConstructor({ type: DataType.StringArray, length: 2 }),
        arrayConstructor({ type: DataType.I32Array, length: 3 }),
        personType,
      ],
      retType: DataType.Void,
    })],
    paramsValue: [func]
  })
  load({
    library: "libsum",
    funcName: "callFunction",
    retType: DataType.Void,
    paramsType: [
      DataType.External,
    ],
    paramsValue: unwrapPointer(funcExternal),
  });
}

const testCpp = () => {
  const classPointer = load({
    library: "libsum",
    funcName: "createMyClassFromC",
    retType: DataType.External,
    paramsType: [
      DataType.String,
      DataType.I32
    ],
    paramsValue: ["classString", 26],
  });
  load({
    library: "libsum",
    funcName: "printMyClass",
    retType: DataType.External,
    paramsType: [
      DataType.External,
    ],
    paramsValue: [classPointer],
  })
  freePointer({
    paramsType: [DataType.External],
    paramsValue: [classPointer],
    pointerType: PointerType.CPointer
  })
}
const testMainProgram = () => {
  if (platform !== 'win32') {
    equal(
      load({
        library: "libnative",
        funcName: "atoi",
        retType: DataType.I32,
        paramsType: [DataType.String],
        paramsValue: ["1000"],
      }),
      1000,
    );
  }
}
const testDefine = () => {
  const res = define({
    sum: {
      library: "libsum",
      retType: DataType.I32,
      paramsType: [DataType.I32, DataType.I32],
    }
  })
  equal(res.sum([1, 2]), 3)
}
const unitTest = () => {
  testNumber()
  logGreen('test number succeed')
  testString()
  logGreen('test string succeed')
  testDefine()
  logGreen('test define succeed')
  testArray()
  logGreen('test array succeed')
  testVoid()
  logGreen('test void succeed')
  testBool()
  logGreen('test bool succeed')
  testMainProgram()
  logGreen('test main program succeed')
  testFunction()
  testCpp()
  logGreen('test cpp succeed')
  testObject()
  logGreen('test object succeed')
  testPointer()
  logGreen('test createPointer succeed')
  testRunInNewThread()
};

unitTest();

exports.unitTest = unitTest;
