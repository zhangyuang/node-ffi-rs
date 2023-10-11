# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## Description

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) is a module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module, but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, which is why ffi-rs was developed.

## benchmark

```bash
$ node bench/bench.js
Running "ffi" suite...
Progress: 100%

  ffi-napi:
    2 007 ops/s, ±9.38%     | slowest, 99.24% slower

  ffi-rs:
    263 846 ops/s, ±0.20%   | fastest

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

- string
- i32(number)
- void(undefined)
- double
- boolean
- i32Array
- stringArray
- doubleArray
- object

## Support Platform

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- win32-x64-msvc
- linux-arm64-gnu
- linux-arm64-musl

## Usage

Here is an example of how to use ffi-rs:

For below c++ code, we compile this file into a dynamic library

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

extern "C" bool return_opposite(bool input) { return !input; }

extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc((size) * sizeof(char *));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}


typedef struct Person {
  const char *name;
  int age;
} Person;

extern "C" const Person *getStruct(const Person *person) {
  printf("Name: %s\n", person->name);
  printf("Age: %d\n", person->age);
  return person;
}

```

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

Then can use `ffi-rs` invoke the dynamic library file contains functions.

```js
const { equal } = require('assert')
const { load, RetType, ParamsType, open, close } = require('ffi-rs')
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
  retType: RetType.I32, // the return value type
  paramsType: [ParamsType.I32, ParamsType.I32], // the parameter types
  paramsValue: [a, b] // the actual parameter values
})
equal(r, a + b)
// release library memory when you're not using it.
close('libsum')

const c = "foo"
const d = "bar"

equal(c + d, load({
  library: 'libsum',
  funcName: 'concatenateStrings',
  retType: RetType.String,
  paramsType: [ParamsType.String, ParamsType.String],
  paramsValue: [c, d]
}))

equal(undefined, load({
  library: 'libsum',
  funcName: 'noRet',
  retType: RetType.Void,
  paramsType: [],
  paramsValue: []
}))

equal(1.1 + 2.2, load({
  library: 'libsum',
  funcName: 'doubleSum',
  retType: RetType.Double,
  paramsType: [ParamsType.Double, ParamsType.Double],
  paramsValue: [1.1, 2.2]
}))

let bigArr = new Array(100000).fill(100)
equal(bigArr[0], load({
  library: 'libsum',
  funcName: 'createArrayi32',
  retType: RetType.I32Array,
  paramsType: [ParamsType.I32Array, ParamsType.I32],
  paramsValue: [bigArr, bigArr.length],
  retTypeLen: bigArr.length
})[0])

let bigDoubleArr = new Array(100).fill(1.1)
equal(bigDoubleArr[0], load({
  library: 'libsum',
  funcName: 'createArrayDouble',
  retType: RetType.DoubleArray,
  paramsType: [ParamsType.DoubleArray, ParamsType.I32],
  paramsValue: [bigDoubleArr, bigDoubleArr.length],
  retTypeLen: bigDoubleArr.length
})[0])

const boolVal = false
equal(!boolVal, load({
  library: 'libsum',
  funcName: 'return_opposite',
  retType: RetType.Boolean,
  paramsType: [ParamsType.Boolean],
  paramsValue: [bool_val],
}))

let stringArr = [c, c.repeat(200)]
equal(stringArr[0], load({
  library: 'libsum',
  funcName: 'createArrayString',
  retType: RetType.StringArray,
  paramsType: [ParamsType.StringArray, ParamsType.I32],
  paramsValue: [stringArr, stringArr.length],
  retTypeLen: stringArr.length
})[0])

const person = {
  name: 'tom',
  age: 23,
}
const personObj = load({
  library: 'libsum',
  funcName: 'getStruct',
  retType: RetType.Object,
  paramsType: [{
    name: ParamsType.String,
    age: ParamsType.I32,
  }],
  paramsValue: [person],
  retFields: {
    name: ParamsType.String,
    age: ParamsType.I32,
  }
})
equal(person.name, personObj.name)
equal(person.age, personObj.age)
```
