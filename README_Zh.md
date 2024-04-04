# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## 简介

`ffi-rs` 是一个使用 `Rust` 编写用于在 `Node.js` 中使用 [ffi](https://en.wikipedia.org/wiki/Foreign_function_interface)来调用 `C++/C/Rust` 等语言的能力。

开发者无需编写 `C++` 代码便可以直接在 `js` 中调用其他语言的能力。此模块在功能上尽量对标[node-ffi](https://github.com/node-ffi/node-ffi)模块，但底层代码已彻底重写。因 `node-ffi` 模块已经多年无人维护处于一个不可用的状态因此开发了`ffi-rs`模块。

## 使用示例

暂时支持 `string/number/void/double` 类型的出参入参类型。根据实际使用场景后续会支持更多的类型。

下面是使用 `ffi-rs` 的一个基本示例。

针对下面的 `c++` 代码，我们将其编译为动态链接库文件

```cpp
#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" void noRet() { printf("%s", "hello world"); }

extern "C" std::vector<int> appendElement(const int *arr, int size) {
  std::vector<int> vec(arr, arr + size);
  vec.push_back(1);
  return vec;
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
const { load, RetType, ParamsType } = require('ffi-rs')
const a = 1
const b = 100

const p = require('ffi-rs')
const r = p.load({
  library: "./libsum.so", // 动态链接库文件
  funcName: 'sum', // 要调用的 method
  retType: RetType.I32, // 返回值的类型
  paramsType: [ParamsType.I32, ParamsType.I32], // 参数的类型
  paramsValue: [a, b] // 实际的参数值
})

expect(r, a + b)

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

```
