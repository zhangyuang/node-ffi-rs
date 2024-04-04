# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## 简介

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) 是一个高性能的使用 `Rust` 编写用于在 `Node.js` 中使用 [ffi](https://en.wikipedia.org/wiki/Foreign_function_interface)来调用 `C++/C/Rust` 等语言的能力。

开发者无需编写 `C++` 代码便可以直接在 `js` 中调用其他语言的能力。此模块在功能上尽量对标[node-ffi](https://github.com/node-ffi/node-ffi)模块，但底层代码已彻底重写。因 `node-ffi` 模块已经多年无人维护处于一个不可用的状态因此开发了`ffi-rs`模块。

## 基准测试

```bash
$ node bench/bench.js
Running "ffi" suite...
Progress: 100%

  ffi-napi:
    2 028 ops/s, ±4.87%     | slowest, 99.24% slower

  ffi-rs:
    287 523 ops/s, ±0.17%   | fastest

Finished 2 cases!
  Fastest: ffi-rs
  Slowest: ffi-napi

```
## 安装

```js
$ npm i ffi-rs
```

## 目前支持的数据类型

目前支持下列类型作为出参入参类型。根据实际使用场景后续会支持更多的类型。

- string
- number(i32)
- void
- double
- boolean
- i32Array
- stringArray
- doubleArray
- object(暂时不支持嵌套对象)
- function(开发中)


## 支持的系统架构

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- win32-x64-msvc
- linux-arm64-gnu
- linux-arm64-musl


## 使用示例

下面是使用 `ffi-rs` 的一个基本示例。

针对下面的 `c++` 代码，我们将其编译为动态链接库文件

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

使用 `ffi-rs` 来调用该动态链接库文件中包含的函数

```js
const { equal } = require('assert')
const { load, DataType, open } = require('ffi-rs')
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

let bigArr = new Array(100).fill(100)
equal(bigArr[0], load({
  library: 'libsum',
  funcName: 'createArrayi32',
  retType: DataType.I32Array,
  paramsType: [DataType.I32Array, DataType.I32],
  paramsValue: [bigArr, bigArr.length],
  retTypeLen: bigArr.length
})[0])

let bigDoubleArr = new Array(100).fill(1.1)
equal(bigDoubleArr[0], load({
  library: 'libsum',
  funcName: 'createArrayDouble',
  retType: DataType.DoubleArray,
  paramsType: [DataType.DoubleArray, DataType.I32],
  paramsValue: [bigDoubleArr, bigDoubleArr.length],
  retTypeLen: bigDoubleArr.length
})[0])

let stringArr = [c, c.repeat(200)]
equal(stringArr[0], load({
  library: 'libsum',
  funcName: 'createArrayString',
  retType: DataType.StringArray,
  paramsType: [DataType.StringArray, DataType.I32],
  paramsValue: [stringArr, stringArr.length],
  retTypeLen: stringArr.length
})[0])
const bool_val = true
equal(!bool_val, load({
  library: 'libsum',
  funcName: 'return_opposite',
  retType: DataType.Boolean,
  paramsType: [DataType.Boolean],
  paramsValue: [bool_val],
}))

const person = {
  name: 'tom',
  age: 23,
}
const personObj = load({
  library: 'libsum',
  funcName: 'getStruct',
  retType: {
    name: DataType.String,
    age: DataType.I32,
  },
  paramsType: [{
    name: DataType.String,
    age: DataType.I32,
  }],
  paramsValue: [person]
})
equal(person.name, personObj.name)
equal(person.age, personObj.age)
```
