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

```js
export const enum RetType {
  String = 0,
  I32 = 1
}
export const enum ParamsType {
  String = 0,
  I32 = 1
}

const p = require('ffi-rs')
const r = p.load({
  library: "/usr/libsum.so", // path to the dynamic library file
  funcName: 'sum', // the name of the function to call
  retType: 1, // the return value type
  paramsType: [1, 1], // the parameter types
  paramsValue: [-99, 2] // the actual parameter values
})

console.log('result', r)
```
