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
} from "./index";

const platform = process.platform;
const dynamicLib = platform === "win32" ? "./sum.dll" : "./libsum.so";

open({
  library: "libsum",
  path: dynamicLib,
});

const unitTest = () => {
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

  const c = "foo";
  const d = c.repeat(200);

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
  const pointer = createPointer({
    paramsType: [DataType.DoubleArray],
    paramsValue: [[1.1, 2.2]]
  })
  const restoreData = restorePointer({
    retType: [arrayConstructor({
      type: DataType.DoubleArray,
      length: 2
    })],
    paramsValue: pointer
  })
  deepStrictEqual(restoreData, [[1.1, 2.2]])

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
  const parent = {
    age: 43,
    doubleArray: [1.1, 2.2, 3.3],
    parent: {},
    doubleProps: 3.3,
    name: "tom father",
    stringArray: ["tom", "father"],
    i32Array: [5, 6, 7],
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
    boolTrue: DataType.Boolean,
    boolFalse: DataType.Boolean,
    longVal: DataType.I64,
    byte: DataType.U8,
    byteArray: arrayConstructor({
      type: DataType.U8Array,
      length: person.byteArray.length,
    }),
  };
  const personObj = load({
    library: "libsum",
    funcName: "getStruct",
    retType: personType,
    paramsType: [
      {
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
          boolTrue: DataType.Boolean,
          boolFalse: DataType.Boolean,
          longVal: DataType.I64,
          byte: DataType.U8,
          byteArray: DataType.U8Array,
        },
        doubleProps: DataType.Double,
        name: DataType.String,
        stringArray: DataType.StringArray,
        i32Array: DataType.I32Array,
        boolTrue: DataType.Boolean,
        boolFalse: DataType.Boolean,
        longVal: DataType.I64,
        byte: DataType.U8,
        byteArray: DataType.U8Array,
      },
    ],
    paramsValue: [person],
  });
  deepStrictEqual(person, personObj);

  const createdPerson = load({
    library: "libsum",
    funcName: "createPerson",
    retType: personType,
    paramsType: [],
    paramsValue: [],
  });
  deepStrictEqual(createdPerson, person);
  let count = 0;
  const func = (a, b, c, d, e, f) => {
    equal(a, 100);
    equal(b, false);
    equal(c, "Hello, World!");
    deepStrictEqual(d, ["Hello", "world"]);
    deepStrictEqual(e, [101, 202, 303]);
    deepStrictEqual(f, person);
    console.log("callback called");
    count++;
    if (count === 2) {
      console.log("test succeed");
      process.exit(0);
    }
  };
  const funcExternal = createPointer({
    paramsType: [funcConstructor({
      paramsType: [
        DataType.I32,
        DataType.Boolean,
        DataType.String,
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
    paramsValue: [funcExternal[0]],
  });
  // load({
  //   library: "libsum",
  //   funcName: "callFunctionDouble",
  //   retType: DataType.Void,
  //   paramsType: [
  //     funcConstructor({
  //       paramsType: [DataType.Double],
  //       retType: DataType.Void,
  //     }),
  //   ],
  //   paramsValue: [func],
  // });

  // cpp
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
};

unitTest();

if (!process.env.MEMORY) {
  close("libsum");
}

exports.unitTest = unitTest;
