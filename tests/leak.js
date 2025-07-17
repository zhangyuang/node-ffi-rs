const { equal, deepStrictEqual } = require("assert");
const {
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
  PointerType,
  isNullPointer,
  FFITypeTag
} = require("../index");
// const { testObject } = require("./struct");
// const { person, personType } = require("./types");

const platform = process.platform;
const dynamicLib = platform === "win32" ? "../sum.dll" : "./libsum.so";
const logGreen = (text) => {
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
  const foo = load({
    library: "libsum",
    funcName: "testbigint",
    retType: DataType.BigInt,
    paramsType: [DataType.BigInt],
    paramsValue: [36028797018963968n],
  })
  equal(typeof foo, "bigint")
  equal(foo.toString(), "36028797018963968")
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
      freeResultMemory: true,
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
      freeResultMemory: true,
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
      freeResultMemory: true,
      paramsType: [arrayConstructor({
        type: DataType.DoubleArray,
        length: bigDoubleArr.length,
      }), DataType.I32],
      paramsValue: [bigDoubleArr, bigDoubleArr.length],
    }),
  );
}
const testPointer = () => {
  // const i32Ptr = createPointer({
  //   paramsType: [DataType.I32],
  //   paramsValue: [100]
  // })
  // const i32Data = restorePointer({
  //   retType: [DataType.I32],
  //   paramsValue: i32Ptr
  // })
  // freePointer({
  //   paramsType: [DataType.I32],
  //   paramsValue: i32Ptr,
  //   pointerType: PointerType.RsPointer
  // })
  // deepStrictEqual(i32Data[0], 100)
  // logGreen('test create and restore i32 pointer success')
  // const stringPointer = createPointer({
  //   paramsType: [DataType.String],
  //   paramsValue: ["foo"]
  // })
  // const stringData = restorePointer({
  //   retType: [DataType.String],
  //   paramsValue: stringPointer
  // })
  // freePointer({
  //   paramsType: [DataType.String],
  //   paramsValue: stringPointer,
  //   pointerType: PointerType.RsPointer
  // })
  // logGreen('test create string pointer success')
  let stringPtr = createPointer({
    paramsType: [DataType.String],
    paramsValue: ["foo"]
  })
  load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: unwrapPointer(stringPtr),
  })
  freePointer({
    paramsType: [DataType.String],
    paramsValue: stringPtr,
    pointerType: PointerType.RsPointer
  })

  const ptr = load({
    library: "libsum",
    funcName: "concatenateStrings",
    retType: DataType.External,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d],
  })
  load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: [ptr],
  })
  freePointer({
    paramsType: [DataType.String],
    paramsValue: wrapPointer([ptr]),
    pointerType: PointerType.RsPointer
  })

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
  const nullPointer = load({
    library: "libsum",
    funcName: "returnNullPointer",
    retType: DataType.External,
    paramsType: [],
    paramsValue: [],
  })
  equal(isNullPointer(nullPointer), true)
  const rsNullPointer = createPointer({
    paramsType: [DataType.Void],
    paramsValue: [undefined]
  })
  equal(isNullPointer(unwrapPointer(rsNullPointer)[0]), true)
  logGreen('test null pointer success')
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
  const funcDesc = funcConstructor({
    paramsType: [
      DataType.I32,
      DataType.Boolean,
      DataType.String,
      DataType.Double,
      arrayConstructor({ type: DataType.StringArray, length: 2 }),
      arrayConstructor({ type: DataType.I32Array, length: 3 }),
      personType,
    ],
    retType: DataType.I32,
  })
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
      paramsType: [funcDesc],
      paramsValue: funcExternal,
      pointerType: PointerType.RsPointer
    })
    return 100
  };
  const funcExternal = createPointer({
    paramsType: [funcDesc],
    paramsValue: [func]
  })
  load({
    library: "libsum",
    funcName: "callFunction",
    // set runInNewThread to true, if you want to get the function return value in c
    // or spawn a new thread in c like
    // std::thread t(threadFunction, func); t.detach();
    runInNewThread: true,
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
    retType: DataType.Void,
    paramsType: [
      DataType.External,
    ],
    paramsValue: [classPointer],
  })
  load({
    library: "libsum",
    funcName: "freeClass",
    retType: DataType.Void,
    paramsType: [DataType.External],
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
  // testNumber()
  // logGreen('test number succeed')
  // testString()
  // logGreen('test string succeed')
  // testDefine()
  // logGreen('test define succeed')
  // testArray()
  // logGreen('test array succeed')
  // testVoid()
  // logGreen('test void succeed')
  // testBool()
  // logGreen('test bool succeed')
  // testMainProgram()
  // logGreen('test main program succeed')
  // testFunction()
  // testCpp()
  // logGreen('test cpp succeed')
  testPointer()
  // logGreen('test createPointer succeed')
  // testRunInNewThread()
  // testObject()
  // logGreen('test object succeed')
  close("libsum")
  global.gc()
  console.log('finish')
};

unitTest();

