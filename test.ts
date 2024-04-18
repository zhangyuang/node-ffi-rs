import { equal, deepStrictEqual } from "assert";
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
  define
} from "./index";

const platform = process.platform;
const dynamicLib = platform === "win32" ? "./sum.dll" : "./libsum.so";
const logGreen = (text: string) => {
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
      paramsValue: [a, b]
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
}
const c = "foo";
const d = c.repeat(200);
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

  let stringArr = [c, c.repeat(20)];
  deepStrictEqual(
    stringArr,
    load({
      library: "libsum",
      funcName: "createArrayString",
      retType: arrayConstructor({
        type: DataType.StringArray,
        length: stringArr.length,
      }),
      paramsType: [DataType.StringArray, DataType.I32],
      paramsValue: [stringArr, stringArr.length],
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
      paramsType: [DataType.I32Array, DataType.I32],
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
      paramsType: [DataType.DoubleArray, DataType.I32],
      paramsValue: [bigDoubleArr, bigDoubleArr.length],
    }),
  );
}
const testCreatePointer = () => {
  const i32Data = restorePointer({
    retType: [DataType.I32],
    paramsValue: createPointer({
      paramsType: [DataType.I32],
      paramsValue: [100]
    })
  })
  deepStrictEqual(i32Data[0], 100)
  logGreen('test i32 pointer success')

  const stringData = restorePointer({
    retType: [DataType.String],
    paramsValue: createPointer({
      paramsType: [DataType.String],
      paramsValue: ["foo"]
    })
  })
  logGreen('test string pointer success')
  const ptr = load({
    library: "libsum",
    funcName: "concatenateStrings",
    retType: DataType.External,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d],
  })
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
  const string = load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: [ptr],
  })
  equal(string, c + d)
  deepStrictEqual(stringData[0], "foo")
  logGreen('string pointer success')
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
  byte: DataType.U8,
  byteArray: arrayConstructor({
    type: DataType.U8Array,
    length: person.byteArray.length,
  }),
};
const testObject = () => {
  const personObjType = {
    age: DataType.I32,
    doubleArray: DataType.DoubleArray,
    parent: {
      parent: {},
      age: DataType.I32,
      doubleProps: DataType.Double,
      name: DataType.String,
      stringArray: DataType.StringArray,
      doubleArray: DataType.DoubleArray,
      i32Array: DataType.I32Array,
      staticBytes: arrayConstructor({
        type: DataType.U8Array,
        length: parent.staticBytes.length,
        dynamicArray: false
      }),
      boolTrue: DataType.Boolean,
      boolFalse: DataType.Boolean,
      longVal: DataType.U64,
      byte: DataType.U8,
      byteArray: DataType.U8Array,
    },
    doubleProps: DataType.Double,
    name: DataType.String,
    stringArray: DataType.StringArray,
    i32Array: DataType.I32Array,
    staticBytes: arrayConstructor({
      type: DataType.U8Array,
      length: person.staticBytes.length,
      dynamicArray: false
    }),
    boolTrue: DataType.Boolean,
    boolFalse: DataType.Boolean,
    longVal: DataType.U64,
    byte: DataType.U8,
    byteArray: DataType.U8Array,
  }
  const createdPerson = load({
    library: "libsum",
    funcName: "createPerson",
    retType: personType,
    paramsType: [],
    paramsValue: [],
  });
  deepStrictEqual(createdPerson, person);
  logGreen('test createdPerson succeed')
  const personObj = load({
    library: "libsum",
    funcName: "getStruct",
    retType: personType,
    paramsType: [
      personObjType
    ],
    paramsValue: [person],
  });
  deepStrictEqual(person, personObj);
  logGreen('test getStruct succeed')
  let personPointer = createPointer({
    paramsType: [personObjType],
    paramsValue: [person]
  })
  const personObjByPointer = load({
    library: "libsum",
    funcName: "getStruct",
    retType: personType,
    paramsType: [
      DataType.External
    ],
    paramsValue: unwrapPointer(personPointer),
  });
  deepStrictEqual(person, personObjByPointer);
  logGreen('test getStructByPointer succeed')
  personPointer = createPointer({
    paramsType: [personObjType],
    paramsValue: [person]
  })
  const restorePersonObjByPointer = restorePointer({
    paramsValue: personPointer,
    retType: [personType]
  })
  deepStrictEqual(person, restorePersonObjByPointer[0])

}

const testFunction = () => {
  let count = 0;
  const func = (a, b, c, d, e, f, g) => {
    equal(a, 100);
    equal(b, false);
    equal(c, "Hello, World!");
    equal(d, "100.11");
    deepStrictEqual(e, ["Hello", "world"]);
    deepStrictEqual(f, [101, 202, 303]);
    deepStrictEqual(g, person);
    console.log("callback called");
    count++;
    if (count === 4) {
      logGreen("test succeed");
      process.exit(0);
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
  });
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
  testArray()
  logGreen('test array succeed')
  testCreatePointer()
  logGreen('test createPointer succeed')
  testVoid()
  logGreen('test void succeed')
  testBool()
  logGreen('test bool succeed')
  testObject()
  logGreen('test object succeed')
  testCpp()
  logGreen('test cpp succeed')
  testMainProgram()
  logGreen('test main program succeed')
  testFunction()
  logGreen('test function succeed')
  testDefine()
  logGreen('test define succeed')
};

unitTest();

if (!process.env.MEMORY) {
  close("libsum");
}

exports.unitTest = unitTest;
