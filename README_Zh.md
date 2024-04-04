# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-APi provides interface (FFI) features for Node.js


## 简介

`ffi-rs` 是一个使用 `Rust` 编写用于在 `Node.js` 中使用 [ffi](https://en.wikipedia.org/wiki/Foreign_function_interface)来调用 `C++/C/Rust` 等语言的能力。

开发者无需编写 `C++` 代码便可以直接在 `js` 中调用其他语言的能力。此模块在功能上尽量对标[node-ffi](https://github.com/node-ffi/node-ffi)模块，但底层代码已彻底重写。因 `node-ffi` 模块已经多年无人维护处于一个不可用的状态因此开发了`ffi-rs`模块。

## 使用示例

暂时只支持 `string/number` 两种类型的出参入参类型。根据实际使用场景后续会支持更多的类型。

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
  library: "/usr/libsum.so", // 动态链接库文件
  funcName: 'sum', // 要调用的 method
  retType: 1, // 返回值的类型
  paramsType: [1, 1], // 参数的类型
  paramsValue: [-99, 2] // 实际的参数值
})

console.log('result', r)

```
