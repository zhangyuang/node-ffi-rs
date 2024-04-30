# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## Description

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) is a high performance module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module, but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, which is why ffi-rs was developed.

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

Currently, ffi-rs only supports there types of parameters and return values. However, support for more types will be added in the future based on actual usage scenarios.

### Basic Type
- [string](#basic-types)
- [i32](#basic-types)
- [i64](#basic-types)
- [void](#basic-types)(undefined)
- [double](#basic-types)
- [boolean](#basic-types)

### Reference Type

- [i32Array](#array)
- [stringArray](#array)
- [doubleArray](#array)
- [object](#struct)(Nested object is also supported at the latest version)
- [function](#function)

## Support Platform

Note: You need to make sure that the compilation environment of the dynamic library is the same as the installation and runtime environment of the `ffi-rs` call.

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- win32-x64-msvc
- linux-arm64-gnu
- linux-arm64-musl

## Usage

Here is an example of how to use ffi-rs:

For below c++ code, we compile this file into a dynamic library

### write c/c++ code

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

### compile c code to dynamic library

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

### call dynamic library by ffi-rs

Then can use `ffi-rs` invoke the dynamic library file contains functions.

### initialization

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

### Use Basic Types

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

### Array

Use `arrayConstructor` to specify array type with legal length which is important.

If the length is incorrect, program maybe exit abnormally

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

### Struct

For create a c struct or get a c struct as a return type, you need to define the types of the parameters strictly in the order in which the fields of the c structure are defined.

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
} Person;

extern "C" Person *getStruct(Person *person) {
  return person;
}

extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  person->doubleArray = (double *)malloc(sizeof(double) * 3);
  person->doubleArray[0] = 1.1;
  person->doubleArray[1] = 2.2;
  person->doubleArray[2] = 3.3;

  // Initialize age and doubleProps
  person->age = 23;
  person->doubleProps = 1.1;

  // Allocate and initialize name
  person->name = strdup("tom");

  person->stringArray = (char **)malloc(sizeof(char *) * 1);
  person->stringArray[0] = strdup("tom");

  person->i32Array = (int *)malloc(sizeof(int) * 4);
  person->i32Array[0] = 1;
  person->i32Array[1] = 2;
  person->i32Array[2] = 3;
  person->i32Array[3] = 4;
  person->boolTrue = true;
  person->boolFalse = false;

  // Allocate and initialize parent
  person->parent = (Person *)malloc(sizeof(Person));
  person->parent->doubleArray = (double *)malloc(sizeof(double) * 3);
  person->parent->doubleArray[0] = 1.1;
  person->parent->doubleArray[1] = 2.2;
  person->parent->doubleArray[2] = 3.3;
  person->parent->age = 43;
  person->parent->doubleProps = 3.3;
  person->parent->name = strdup("tom father");
  person->parent->stringArray = (char **)malloc(sizeof(char *) * 2);
  person->parent->stringArray[0] = strdup("tom");
  person->parent->stringArray[1] = strdup("father");
  person->parent->i32Array = (int *)malloc(sizeof(int) * 3);
  person->parent->i32Array[0] = 5;
  person->parent->i32Array[1] = 6;
  person->parent->i32Array[2] = 7;
  person->parent->boolTrue = true;
  person->parent->boolFalse = false;

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
      },
      doubleProps: DataType.Double,
      name: DataType.String,
      stringArray: DataType.StringArray,
      i32Array: DataType.I32Array,
      boolTrue: DataType.Boolean,
      boolFalse: DataType.Boolean,
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

Attention，since the vast majority of scenaros developers pass js function to c as a callback, so `ffi-rs` will create [threadsafe_function](https://nodejs.org/api/n-api.html#napi_threadsafe_function) from jsfunction which means the jsfunction will be called asynchronous, and Node.js process will not be exited automatically
