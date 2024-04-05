# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-API provides interface (FFI) features for Node.js


## Description

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) is a high-performance module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, so ffi-rs was developed to fill that void.

## features

- High performance ✨
- Simpler data description and API interface 💗
- Support more data types between `Node.js` and `c type` 😊
- Support modify data in place 🥸

## benchmark

```bash
$ node bench/bench.js
Running "ffi" suite...
Progress: 100%

  ffi-napi:
    2 028 ops/s, ±4.87%     | slowest, 99.24% slower

  ffi-rs:
    318 467 ops/s, ±0.17%   | fastest

Finished 2 cases!
  Fastest: ffi-rs
  Slowest: ffi-napi

```

## install

```js
$ npm i ffi-rs
```

## Support type

Currently, ffi-rs only supports these types of parameters and return values. However, support for more types may be added in the future based on actual usage scenarios.

### Basic Type
- [string](#basic-types)
- [u8](#basic-types)
- [i32](#basic-types)
- [i64](#basic-types)
- [u64](#basic-types)
- [void](#basic-types)(undefined)
- [double](#basic-types)
- [boolean](#basic-types)

### Reference Type

- [pointer](#pointer)
- [u8Array](#buffer)(buffer)
- [i32Array](#array)
- [stringArray](#array)
- [doubleArray](#array)
- [object](#struct)(Nested object is also supported at the latest version)
- [function](#function)

### C++ Class

If you want to call C++ function, see [tutorial](#C++)

## Support Platform

Note: You need to make sure that the compilation environment of the dynamic library is the same as the installation and runtime environment of the `ffi-rs` call.

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- linux-x64-musl
- win32-x64-msvc
- win32-ia32-msvc
- linux-arm64-gnu
- linux-arm64-musl

## Usage

View [test.ts](./test.ts) get the latest usage

Here is an example of how to use ffi-rs:

For the following C++ code, we compile this file into a dynamic library

### write C/C++ code

Note: The return value type of a function must be of type c

```cpp
#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" void noRet() { printf("%s", "hello world"); }
extern "C" bool return_opposite(bool input) { return !input; }


```

### compile C code into a dynamic library

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

### call dynamic library by ffi-rs

Then you can use `ffi-rs` to invoke the dynamic library file that contains functions.

### Initialization

```js
const { equal } = require('assert')
const { load, DataType, open, close, arrayConstructor } = require('ffi-rs')
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"
// first open dynamic library with key for close
// It only needs to be opened once.
open({
  library: 'libsum', // key
  path: dynamicLib // path
})
const r = load({
  library: "libsum", // path to the dynamic library file
  funcName: 'sum', // the name of the function to call
  retType: DataType.I32, // the return value type
  paramsType: [DataType.I32, DataType.I32], // the parameter types
  paramsValue: [a, b] // the actual parameter values
})
equal(r, a + b)
// release library memory when you're not using it.
close('libsum')

```

### Load Main Program handle

You can alse pass emptry path string in `open` function like [ffi-napi](https://github.com/node-ffi-napi/node-ffi-napi?tab=readme-ov-file#example) to get the main program handle refer [dlopen](https://man7.org/linux/man-pages/man3/dlopen.3.html)

```js
open({
  library: "libnative",
  path: "",
});
// Call atoi function which is includeed in the basic c library
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
```

### Basic Types

`number|string|boolean|double|void` are basic types

```js
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
const bool_val = true
equal(!bool_val, load({
  library: 'libsum',
  funcName: 'return_opposite',
  retType: DataType.Boolean,
  paramsType: [DataType.Boolean],
  paramsValue: [bool_val],
}))
```

### Buffer

In the latest version, `ffi-rs` supports modifying data in place.

The sample code is as follows

```c
extern int modifyData(char* buffer) {
    // modify buffer data in place
}
```

```js
const arr = Buffer.alloc(200) // create buffer
const res = load({
  library: "libsum",
  funcName: "modifyData",
  retType: DataType.I32,
  paramsType: [
    DataType.U8Array
  ],
  paramsValue: [arr]
})
console.log(arr) // buffer data can be updated
```

### Array

When using `array` as `retType`, you should use `arrayConstructor` to specify the array type with a legal length which is important.

If the length is incorrect, the program may exit abnormally

```cpp
extern "C" int *createArrayi32(const int *arr, int size) {
  int *vec = (int *)malloc((size) * sizeof(int));

  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}
extern "C" double *createArrayDouble(const double *arr, int size) {
  double *vec = (double *)malloc((size) * sizeof(double));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc((size) * sizeof(char *));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

```

```js
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

```

### Pointer

In `ffi-rs`, we use [DataType.External](https://nodejs.org/api/n-api.html#napi_create_external) for wrapping the `pointer` which enables it to be passed between `Node.js` and `C`.

```cpp
extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" char *getStringFromPtr(void *ptr) { return (char *)ptr; };
```


```js
// get pointer
const ptr = load({
  library: "libsum",
  funcName: "concatenateStrings",
  retType: DataType.External,
  paramsType: [DataType.String, DataType.String],
  paramsValue: [c, d],
})

// send pointer
const string = load({
  library: "libsum",
  funcName: "getStringFromPtr",
  retType: DataType.String,
  paramsType: [DataType.External],
  paramsValue: [ptr],
})
```

#### CreatePointer

```js
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
```

For the code above, we can use `createPointer` function to wrap a pointer data and send it as paramsValue

```js
const funcExternal: unknown[] = createPointer({
  paramsType: [DataType.DoubleArray],
  paramsValue: [[1.1,2.2]]
})
const ptr = funcExternal[0]
load({
  library: "libsum",
  funcName: "createArrayDouble",
  retType: arrayConstructor({
    type: DataType.DoubleArray,
    length: bigDoubleArr.length,
  }),
  paramsType: [DataType.External, DataType.I32],
  paramsValue: [ptr, bigDoubleArr.length],
})
```

The two pieces of code above are equivalent

Similarly, you can use `restorePointer` to restore data from `pointer` which is wrapped by `createPointer`

```js
const pointerArr = createPointer({
  paramsType: [DataType.DoubleArray],
  paramsValue: [[1.1, 2.2]]
})
const restoreData = restorePointer({
  retType: [arrayConstructor({
    type: DataType.DoubleArray,
    length: 2
  })],
  paramsValue: pointerArr
})
deepStrictEqual(restoreData, [[1.1, 2.2]])
```

### Struct

To create a c struct or get a c struct as a return type, you need to define the types of the parameters strictly in the order in which the fields of the c structure are defined.

```cpp
typedef struct Person {
  int age;
  double *doubleArray;
  Person *parent;
  double doubleProps;
  const char *name;
  char **stringArray;
  int *i32Array;
  bool boolTrue;
  bool boolFalse;
  int64_t longVal;
  char byte;
  char *byteArray;
} Person;
extern "C" Person *getStruct(Person *person) {
  return person;
}

extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  double initDoubleArray[] = {1.1, 2.2, 3.3};
  person->doubleArray = (double *)malloc(sizeof(initDoubleArray));
  memcpy(person->doubleArray, initDoubleArray, sizeof(initDoubleArray));

  // Initialize age and doubleProps
  person->age = 23;
  person->doubleProps = 1.1;
  person->byte = 'A';

  // Allocate and initialize name
  person->name = strdup("tom");

  char *stringArray[] = {strdup("tom")};
  person->stringArray = (char **)malloc(sizeof(stringArray));
  memcpy(person->stringArray, stringArray, sizeof(stringArray));

  // Allocate and initialize byteArray
  char initByteArray[] = {101, 102};
  person->byteArray = (char *)malloc(sizeof(initByteArray));
  memcpy(person->byteArray, initByteArray, sizeof(initByteArray));

  int initI32Array[] = {1, 2, 3, 4};
  person->i32Array = (int *)malloc(sizeof(initI32Array));
  memcpy(person->i32Array, initI32Array, sizeof(initI32Array));

  person->boolTrue = true;
  person->boolFalse = false;
  person->longVal = 4294967296;

  // Allocate and initialize parent
  person->parent = (Person *)malloc(sizeof(Person));
  double parentDoubleArray[] = {1.1, 2.2, 3.3};
  person->parent->doubleArray = (double *)malloc(sizeof(parentDoubleArray));
  memcpy(person->parent->doubleArray, parentDoubleArray,
         sizeof(parentDoubleArray));

  person->parent->age = 43;
  person->parent->doubleProps = 3.3;
  person->parent->name = strdup("tom father");

  char *pstringArray[] = {strdup("tom"), strdup("father")};
  person->parent->stringArray = (char **)malloc(sizeof(pstringArray));

  memcpy(person->parent->stringArray, pstringArray, sizeof(pstringArray));

  int parentI32Array[] = {5, 6, 7};
  person->parent->i32Array = (int *)malloc(sizeof(parentI32Array));
  memcpy(person->parent->i32Array, parentI32Array, sizeof(parentI32Array));

  person->parent->boolTrue = true;
  person->parent->boolFalse = false;
  person->parent->longVal = 5294967296;
  person->parent->byte = 'B';

  char parentByteArray[] = {103, 104};
  person->parent->byteArray = (char *)malloc(sizeof(parentByteArray));
  memcpy(person->parent->byteArray, parentByteArray, sizeof(parentByteArray));

  return person;
}
```

```js
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

```

## Function

`ffi-rs` supports passing js function to c, like this

```cpp
typedef void (*FunctionPointer)(int a, bool b, char *c, char **d, int *e,
                                Person *p);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction\n");

  for (int i = 0; i < 2; i++) {
    int a = 100;
    bool b = false;
    double ddd = 100.11;
    char *c = (char *)malloc(14 * sizeof(char));
    strcpy(c, "Hello, World!");

    char **stringArray = (char **)malloc(sizeof(char *) * 2);
    stringArray[0] = strdup("Hello");
    stringArray[1] = strdup("world");

    int *i32Array = (int *)malloc(sizeof(int) * 3);
    i32Array[0] = 101;
    i32Array[1] = 202;
    i32Array[2] = 303;

    Person *p = createPerson();
    func(a, b, c, stringArray, i32Array, p);
  }
}
```

Corresponds to the code above，you can use `ffi-rs` like

```js
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

load({
  library: "libsum",
  funcName: "callFunction",
  retType: DataType.Void,
  paramsType: [
    funcConstructor({
      paramsType: [
        DataType.I32,
        DataType.Boolean,
        DataType.String,
        arrayConstructor({ type: DataType.StringArray, length: 2 }),
        arrayConstructor({ type: DataType.I32Array, length: 3 }),
        personType,
      ],
      retType: DataType.Void,
    }),
  ],
  paramsValue: [func],
});
```

The function parameters supports type are all in the example above (double type is unsupported at this time), we will support more types in the future

Attention，since the vast majority of scenarios developers pass js function to c as a callback, so `ffi-rs` will create [threadsafe_function](https://nodejs.org/api/n-api.html#napi_threadsafe_function) from jsfunction which means the jsfunction will be called asynchronous, and Node.js process will not be exited automatically


## C++

We'll provide more examples from real-world scenarios, if you have any ideas, please submit an issue

### class type

In C++ scene, we can use `DataType.External` to get a class type pointer

In the code below, we use C types to wrap C++ types such as convert `char *` to `std::string` and return class pointer

```cpp
MyClass *createMyClass(std::string name, int age) {
  return new MyClass(name, age);
}

extern "C" MyClass *createMyClassFromC(const char *name, int age) {
  return createMyClass(std::string(name), age);
}

extern "C" void printMyClass(MyClass *instance) { instance->print(); }
```

And then, it can called by the following code

```js
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
```
