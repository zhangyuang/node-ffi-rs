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
- function


## 支持的系统架构

注意：你需要保证动态链接库的编译环境，与调用 `ffi-rs` 的安装环境和运行环境一致

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- win32-x64-msvc
- linux-arm64-gnu
- linux-arm64-musl


### 编写 c/cpp 代码

注意：返回的数据类型必须是属于 c 类型的而不是 c++ 类型

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

### 将 c/cpp 代码编译为动态链接库

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

### 使用 ffi-rs 来调用动态链接库

Then can use `ffi-rs` invoke the dynamic library file contains functions.

### 初始化

```js
const { equal } = require('assert')
const { load, DataType, open, close, arrayConstructor } = require('ffi-rs')
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"
// 首先你需要调用 open 来打开一个动态链接库并指定一个key来作为标志符在后续操作里调用
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
// 当你不需要再用到这个动态链接库时，使用close来释放它
close('libsum')

```

### 基本类型

`number|string|boolean|double|void` 属于基本类型

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

使用 `arrayConstructor` 来创建数组的类型描述。指定返回值中数组的长度是非常重要的，如果输入了不争取的长度可能会引发程序异常退出。

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

创建一个 c 的结构体或者将 c 结构体类型作为返回值，你需要严格按照 c 结构体中声明的字段顺序来定义 js 侧参数的顺序。

```cpp
typedef struct Person {
  const char *name;
  int age;
  double doubleProps;
  char **stringArray;
  double *doubleArray;
  int *i32Array;
} Person;

extern "C" const Person *getStruct(const Person *person) {
  return person;
}
extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  person->doubleArray = (double *)malloc(sizeof(double) * 3);
  person->doubleArray[0] = 1.0;
  person->doubleArray[1] = 2.0;
  person->doubleArray[2] = 3.0;

  // Initialize age and doubleProps
  person->age = 30;
  person->doubleProps = 1.23;

  // Allocate and initialize name
  person->name = strdup("John Doe");

  person->stringArray = (char **)malloc(sizeof(char *) * 2);
  person->stringArray[0] = strdup("Hello");
  person->stringArray[1] = strdup("World");

  person->i32Array = (int *)malloc(sizeof(int) * 3);
  person->i32Array[0] = 1;
  person->i32Array[1] = 2;
  person->i32Array[2] = 3;
  person->testnum = 123;
  person->boolTrue = true;
  person->boolFalse = false;

  return person;
}
```

```js
const person = {
  doubleArray: [1.1, 2.2, 3.3],
  age: 23,
  doubleProps: 1.1,
  name: 'tom',
  stringArray: ["foo", "bar"],
  i32Array: [1, 2, 3, 4],
  testnum: 32,
  boolTrue: true,
  boolFalse: false
}
const personObj = load({
  library: 'libsum',
  funcName: 'getStruct',
  retType: {
    doubleArray: arrayConstructor({ type: DataType.DoubleArray, length: person.doubleArray.length }),
    age: DataType.I32,
    doubleProps: DataType.Double,
    name: DataType.String,
    stringArray: arrayConstructor({ type: DataType.StringArray, length: person.stringArray.length }),
    i32Array: arrayConstructor({ type: DataType.I32Array, length: person.i32Array.length }),
    testnum: DataType.I32,
    boolTrue: DataType.Boolean,
    boolFalse: DataType.Boolean,
  },
  paramsType: [{
    age: DataType.I32,
    doubleProps: DataType.Double,
    name: DataType.String,
    stringArray: DataType.StringArray,
    doubleArray: DataType.DoubleArray,
    i32Array: DataType.I32Array,
    testnum: DataType.I32,
    boolTrue: DataType.Boolean,
    boolFalse: DataType.Boolean,
  }],
  paramsValue: [person]
})
deepStrictEqual(person, personObj)
const p = load({
  library: 'libsum',
  funcName: 'createPerson',
  retType: {
    doubleArray: arrayConstructor({ type: DataType.DoubleArray, length: 3 }),
    age: DataType.I32,
    doubleProps: DataType.Double,
    name: DataType.String,
    stringArray: arrayConstructor({ type: DataType.StringArray, length: 2 }),
    i32Array: arrayConstructor({ type: DataType.I32Array, length: 3 }),
    testnum: DataType.I32,
    boolTrue: DataType.Boolean,
    boolFalse: DataType.Boolean,
  },
  paramsType: [],
  paramsValue: []
})
console.log('createPerson', p)
deepStrictEqual(p, {
  doubleArray: [1, 2, 3],
  age: 30,
  doubleProps: 1.23,
  name: 'John Doe',
  stringArray: ['Hello', 'World'],
  i32Array: [1, 2, 3],
  testnum: 123,
  boolTrue: true,
  boolFalse: false
})

```

## Function

`ffi-rs` 支持传递 js 函数给 c 语言侧，就像这样

```cpp
typedef void (*FunctionPointer)(int a, bool b, char *c, char **d, int *e);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction\n");
  int a = 100;
  bool b = false;
  char *c = (char *)malloc(14 * sizeof(char));
  strcpy(c, "Hello, World!");
  char **stringArray = (char **)malloc(sizeof(char *) * 2);
  stringArray[0] = strdup("Hello");
  stringArray[1] = strdup("world");
  int *i32Array = (int *)malloc(sizeof(int) * 3);
  i32Array[0] = 101;
  i32Array[1] = 202;
  i32Array[2] = 303;
  func(a, b, c, stringArray, i32Array);
}
```

与上面的代码相对应，你可以这样使用 `ffi-rs` 来传递

```js
const func = (a, b, c, d, e) => {
  console.log('func params', a, b, c, d, e)
  equal(a, 100)
  equal(b, false)
  equal(c, 'Hello, World!')
  deepStrictEqual(d, ['Hello', 'world'])
  deepStrictEqual(e, [101, 202, 303])
}

load({
  library: 'libsum',
  funcName: 'callFunction',
  retType: DataType.Void,
  paramsType: [funcConstructor({
    paramsType: [DataType.I32, DataType.Boolean, DataType.String,
    arrayConstructor({ type: DataType.StringArray, length: 2 }),
    arrayConstructor({ type: DataType.I32Array, length: 3 }),
    ],
    retType: DataType.Void
  })],
  paramsValue: [func],
})
```

目前函数支持的参数类型都在上面的例子里，我们将会在未来支持更多的参数类型
