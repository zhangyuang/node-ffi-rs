# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## Description

ffi-rs is a module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module, but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, which is why ffi-rs was developed.

## install

```js
$ npm i ffi-rs
```

## Support type

Currently, ffi-rs only supports there types of parameters and return values. However, support for more types will be added in the future based on actual usage scenarios.

- string
- number(i32)
- void
- double
- i32Array
- stringArray
- doubleArray

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
extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc((size) * sizeof(char *));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
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
const { load, RetType, ParamsType } = require('ffi-rs')
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"

const r = load({
  library: "./libsum.so", // path to the dynamic library file
  funcName: 'sum', // the name of the function to call
  retType: RetType.I32, // the return value type
  paramsType: [ParamsType.I32, ParamsType.I32], // the parameter types
  paramsValue: [a, b] // the actual parameter values
})

equal(r, a + b)

const c = "foo"
const d = "bar"

equal(c + d, load({
  library: dynamicLib,
  funcName: 'concatenateStrings',
  retType: RetType.String,
  paramsType: [ParamsType.String, ParamsType.String],
  paramsValue: [c, d]
}))

equal(undefined, load({
  library: dynamicLib,
  funcName: 'noRet',
  retType: RetType.Void,
  paramsType: [],
  paramsValue: []
}))

equal(1.1 + 2.2, load({
  library: dynamicLib,
  funcName: 'doubleSum',
  retType: RetType.Double,
  paramsType: [ParamsType.Double, ParamsType.Double],
  paramsValue: [1.1, 2.2]
}))

let bigArr = new Array(100000).fill(100)
equal(bigArr[0], load({
  library: dynamicLib,
  funcName: 'createArrayi32',
  retType: RetType.I32Array,
  paramsType: [ParamsType.I32Array, ParamsType.I32],
  paramsValue: [bigArr, bigArr.length],
  retTypeLen: bigArr.length
})[0])

let bigDoubleArr = new Array(100).fill(1.1)
equal(bigDoubleArr[0], load({
  library: dynamicLib,
  funcName: 'createArrayDouble',
  retType: RetType.DoubleArray,
  paramsType: [ParamsType.DoubleArray, ParamsType.I32],
  paramsValue: [bigDoubleArr, bigDoubleArr.length],
  retTypeLen: bigDoubleArr.length
})[0])


let stringArr = [c, c.repeat(200)]
equal(stringArr[0], load({
  library: dynamicLib,
  funcName: 'createArrayString',
  retType: RetType.StringArray,
  paramsType: [ParamsType.StringArray, ParamsType.I32],
  paramsValue: [stringArr, stringArr.length],
  retTypeLen: stringArr.length
})[0])
```
