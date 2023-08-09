# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## Description

ffi-rs is a module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module, but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, which is why ffi-rs was developed.

## Usage

Currently, ffi-rs only supports two types of parameters and return values: strings and numbers. However, support for more types will be added in the future based on actual usage scenarios.

Here is an example of how to use ffi-rs:

For below c++ code, we compile this file into a dynamic library

```cpp
extern "C" int sum(int a, int b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
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
  library: "./libsum.so",
  funcName: 'concatenateStrings',
  retType: ParamsType.String,
  paramsType: [ParamsType.String, ParamsType.String],
  paramsValue: [c, d]
}))


```
