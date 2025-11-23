# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

一个用Rust和N-API编写的模块, 为Node.js提供外部函数接口(FFI)功能，中文文档的更新不一定及时，建议阅读[英文文档](./README.md)以便获取最新的信息

<div align="">
<a href="https://github.com/zhangyuang/node-ffi-rs/actions" target="_blank"><img src="https://github.com/zhangyuang/node-ffi-rs/workflows/CI/badge.svg" alt="githubActions" /></a>
<a href="https://www.npmjs.com/package/ffi-rs" target="_blank"><img src="https://img.shields.io/npm/dm/ffi-rs">
</a>
</div>

## 描述

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs)是一个用Rust和N-API编写的高性能模块, 为Node.js提供FFI(外部函数接口)功能。它允许开发者直接从JavaScript调用用其他语言如C++、C和Rust编写的函数, 而无需编写任何C++代码。

该模块旨在提供类似于node-ffi模块的功能, 但底层代码库完全重写。node-ffi模块已经多年未维护, 不再可用, 所以开发了ffi-rs来填补这个空白。

## 特性

* 高性能 ✨
* 更好的类型提示 🧐
* 更简单的数据描述和API接口 💗
* 支持`Node.js`和`c`之间更多不同的数据类型 😊
* 支持原地修改数据 🥸
* 提供多种方式直接处理指针类型 🐮
* 支持[在新线程中](#runInNewThread)运行ffi任务 🤩️
* 支持输出[errno](#errno)信息 🤔️
* 无需使用[ref](https://www.npmjs.com/package/ref)来处理指针 🤫

## 基准测试

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

## 更新日志

查看[CHANGELOG.md](./CHANGELOG.md)

## 生态系统

[abstract-socket-rs](https://github.com/zhangyuang/abstract-socket-rs)

## 安装

```js
$ npm i ffi-rs
```

## 支持的类型

目前, ffi-rs仅支持这些类型的参数和返回值。然而, 根据实际使用场景, 未来可能会添加对更多类型的支持。

### 基本类型

* [string](#基本类型)
* [wideString](#基本类型)
* [u8](#基本类型)
* [i16](#基本类型)
* [i32](#基本类型)
* [i64](#基本类型)
* [bigInt](#基本类型)
* [u64](#基本类型)
* [u32](#基本类型)
* [void](#基本类型)(类似js的undefined)
* [float](#基本类型)
* [double](#基本类型)
* [boolean](#基本类型)

### 引用类型

* [pointer](#指针)
* [u8Array](#缓冲区)(buffer)
* [i16Array](#数组)
* [i32Array](#数组)
* [stringArray](#数组)
* [doubleArray](#数组)
* [floatArray](#数组)(只能用作paramsType而不能用作retType)
* [object](#结构体)(最新版本也支持嵌套对象)
* [function](#函数)

### C++类

如果你想调用参数类型为类的C++函数, 你可以使用 `pointer` 类型, 参见[教程](#C++)

## 支持的平台

注意: 你需要确保动态库的编译环境与 `ffi-rs` 调用的安装和运行环境相同。

* darwin-x64
* darwin-arm64
* linux-x64-gnu
* linux-x64-musl
* win32-x64-msvc
* win32-ia32-msvc
* win32-arm64-msvc
* linux-arm64-gnu
* linux-arm64-musl
* linux-arm-gnueabihf
* android-arm64

## 赞助

There are two ways to sponsor me both Alipay and WeChat

Eth address: 0x87a2575a5d4dbD5f965e3e3a3d20641BC9a5d192

<div style="display:flex">
 <img src="https://doc.ssr-fc.com/images/wepay.jpg" width=200>
  <img src="https://doc.ssr-fc.com/images/alipay.jpg" width=200>
</div>

## 使用方法

查看[tests/index.ts](./tests/index.ts)获取最新用法

以下是如何使用ffi-rs的示例:

对于以下C++代码, 我们将此文件编译成动态库

### 编写外部函数代码

注意: 函数的返回值类型必须是c类型

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
extern "C" uint32_t testU32(uint32_t a, uint32_t b) { return a + b; }

```

### 将C代码编译成动态库

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

### 使用ffi-rs调用动态库

然后你可以使用 `ffi-rs` 调用包含函数的动态库文件。

### 初始化

建议使用typescript开发以获得类型提示

```js
const {
    equal
} = require('assert')
const {
    load,
    DataType,
    open,
    close,
    arrayConstructor,
    define
} = require('ffi-rs')
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"
// 首先用key打开动态库以便关闭
// 只需打开一次。
open({
    library: 'libsum', // key
    path: dynamicLib // 路径
})
const r = load({
    library: "libsum", // 动态库文件的路径
    funcName: 'sum', // 要调用的函数名
    retType: DataType.I32, // 返回值类型
    paramsType: [DataType.I32, DataType.I32], // 参数类型
    paramsValue: [a, b] // 实际参数值
    // freeResultMemory: true, // 是否需要自动释放返回值的内存,默认为false
})
equal(r, a + b)
// 当你不再使用库时释放库内存。
close('libsum')

// 使用define函数定义函数签名
const res = define({
    sum: {
        library: "libsum",
        retType: DataType.I32,
        paramsType: [DataType.I32, DataType.I32],
    },
    atoi: {
        library: "libnative",
        retType: DataType.I32,
        paramsType: [DataType.String],
        paramsValue: ["1000"],
    }
})
equal(res.sum([1, 2]), 3)
equal(res.atoi(["1000"]), 1000)
```

### 加载主程序句柄

你也可以在 `open` 函数中传递空路径字符串, 像[ffi-napi](https://github.com/node-ffi-napi/node-ffi-napi?tab=readme-ov-file#example)那样获取主程序句柄, 参考[dlopen](https://man7.org/linux/man-pages/man3/dlopen.3.html)

```js
open({
    library: "libnative",
    path: "",
});
// 在darwin/linux中,你可以调用包含在基本c库中的atoi函数
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

### 基本类型

`number|string|boolean|double|void` 是基本类型

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

// U32 类型用法
equal(3147483647, load({
    library: 'libsum',
    funcName: 'testU32',
    retType: DataType.U32,
    paramsType: [DataType.U32, DataType.U32],
    paramsValue: [2147483647, 1000000000],
}))
```

### 缓冲区

在最新版本中, `ffi-rs` 支持原地修改数据。

示例代码如下

```c
extern int modifyData(char* buffer) {
    // 原地修改buffer数据
}
```

```js
const arr = Buffer.alloc(200) // 创建buffer
const res = load({
    library: "libsum",
    funcName: "modifyData",
    retType: DataType.I32,
    paramsType: [
        DataType.U8Array
    ],
    paramsValue: [arr]
})
console.log(arr) // buffer数据可以被更新
```

### 数组

当使用 `array` 作为 `retType` 时, 你应该使用 `arrayConstructor` 指定数组类型和合法长度, 这很重要。

如果长度不正确, 程序可能会异常退出

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
    retType: arrayConstructor({
        type: DataType.I32Array,
        length: bigArr.length
    }),
    paramsType: [DataType.I32Array, DataType.I32],
    paramsValue: [bigArr, bigArr.length],
}))

let bigDoubleArr = new Array(5).fill(1.1)
deepStrictEqual(bigDoubleArr, load({
    library: 'libsum',
    funcName: 'createArrayDouble',
    retType: arrayConstructor({
        type: DataType.DoubleArray,
        length: bigDoubleArr.length
    }),
    paramsType: [DataType.DoubleArray, DataType.I32],
    paramsValue: [bigDoubleArr, bigDoubleArr.length],
}))
let stringArr = [c, c.repeat(20)]

deepStrictEqual(stringArr, load({
    library: 'libsum',
    funcName: 'createArrayString',
    retType: arrayConstructor({
        type: DataType.StringArray,
        length: stringArr.length
    }),
    paramsType: [DataType.StringArray, DataType.I32],
    paramsValue: [stringArr, stringArr.length],
}))
```

### 指针

在 `ffi-rs` 中, 我们使用[DataType. External](https://nodejs.org/api/n-api.html#napi_create_external)来包装 `pointer` , 使其能够在 `Node.js` 和 `C` 之间传递。

`Pointer` 是复杂和底层的, `ffi-rs` 提供了四个函数来处理这个指针, 包括 `createPointer` 、 `restorePointer` 、 `unwrapPointer` 、 `wrapPointer` 、 `freePointer` , 用于不同的场景。

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
// 获取指针
const ptr = load({
    library: "libsum",
    funcName: "concatenateStrings",
    retType: DataType.External,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d],
})

// 发送指针
const string = load({
    library: "libsum",
    funcName: "getStringFromPtr",
    retType: DataType.String,
    paramsType: [DataType.External],
    paramsValue: [ptr],
})
```

#### createPointer

`createPointer` 函数用于创建指向指定类型的指针。为了避免错误, 开发者必须理解这个指针是什么类型。

对于像 `i32|u8|i64|f64` 这样的数值类型, createPointer将创建一个像 `*mut i32` 这样指向这些数字的指针

对于原本就是指针类型的类型, 如在 `c` 中表示 `string` 类型的 `char *` , createPointer将创建一个像 `*mut *mut c_char` 这样指向 `*mut c_char` 的双重指针。开发者可以使用 `unwrapPointer` 获取内部指针 `*mut c_char`

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

对于上面的代码, 我们可以使用 `createPointer` 函数来包装一个指针数据并将其作为paramsValue发送

```js
const ptrArr: unknown[] = createPointer({
    paramsType: [DataType.DoubleArray],
    paramsValue: [
        [1.1, 2.2]
    ]
})

load({
    library: "libsum",
    funcName: "createArrayDouble",
    retType: arrayConstructor({
        type: DataType.DoubleArray,
        length: bigDoubleArr.length,
    }),
    paramsType: [DataType.External, DataType.I32],
    paramsValue: [unwrapPointer(ptrArr)[0], bigDoubleArr.length],
})
```

上面两段代码是等效的

#### restorePointer

同样, 你可以使用 `restorePointer` 从由 `createPointer` 包装或作为外部函数返回值的 `pointer` 中恢复数据

```js
const pointerArr = createPointer({
    paramsType: [DataType.DoubleArray],
    paramsValue: [
        [1.1, 2.2]
    ]
})
const restoreData = restorePointer({
    retType: [arrayConstructor({
        type: DataType.DoubleArray,
        length: 2
    })],
    paramsValue: pointerArr
})
deepStrictEqual(restoreData, [
    [1.1, 2.2]
])
```

#### freePointer

`freePointer` 用于释放不会自动释放的内存。

默认情况下, `ffi-rs` 会为ffi调用参数和返回结果释放数据内存以防止内存泄漏。除了以下情况。

* 调用`load`方法时设置`freeResultMemory: false`

如果你将freeResultMemory设置为false, `ffi-rs` 将不会释放在c环境中分配的返回结果内存

* 使用`DataType.External`作为paramsType或retType

如果开发者使用 `DataType.External` 作为paramsType或retType, 请使用 `freePointer` 释放指针的内存。参考[test.ts](./test.ts#170)

#### wrapPointer

`wrapPointer` 用于创建多重指针。

例如, 开发者可以使用 `wrapPointer` 创建一个指向其他现有指针的指针。

```js
const {
    wrapPointer
} = require('ffi-rs')
// ptr类型是*mut c_char
const ptr = load({
    library: "libsum",
    funcName: "concatenateStrings",
    retType: DataType.External,
    paramsType: [DataType.String, DataType.String],
    paramsValue: [c, d],
})

// wrapPtr类型是*mut *mut c_char
const wrapPtr = wrapPointer([ptr])[0]
```

#### unwrapPointer

`unwrapPointer` 与 `wrapPointer` 相反, 用于获取多重指针的内部指针

```js
const {
    unwrapPointer,
    createPointer
} = require('ffi-rs')
// ptr类型是*mut *mut c_char
let ptr = createPointer({
    paramsType: [DataType.String],
    paramsValue: ["foo"]
})

// unwrapPtr类型是*mut c_char
const unwrapPtr = unwrapPointer([ptr])[0]
```

### 结构体

要创建c结构体或获取c结构体作为返回类型, 你需要严格按照c结构体字段定义的顺序定义参数类型。

`ffi-rs` 在[sum.cpp](https://github.com/zhangyuang/node-ffi-rs/blob/master/cpp/sum.cpp#L48)中提供了一个名为 `Person` 的c结构体, 包含多种类型的字段

关于如何调用外部函数来创建 `Person` 结构体或使用 `Person` 结构体作为返回值的示例调用方法在[这里](https://github.com/zhangyuang/node-ffi-rs/blob/master/test.ts#L289)

#### 在结构体中使用数组

c语言中有两种类型的数组, 如 `int* array` 和 `int array[100]` , 它们有一些不同的用法。

第一种类型 `int* array` 是一个指针类型, 存储数组的第一个地址。

第二种类型 `int array[100]` 是一个固定长度的数组, 数组中的每个元素都有连续的地址。

如果你使用数组作为函数参数, 这通常会传递一个数组指针, 无论你定义的是哪种类型。但如果数组类型在结构体中定义, 两种数组定义会导致结构体的大小和对齐不同。

因此, `ffi-rs` 需要区分这两种类型。

默认情况下, `ffi-rs` 使用指针数组来计算结构体。如果你确认应该使用静态数组, 你可以按以下方式定义它

```js
typedef struct Person {
    //...
    uint8_t staticBytes[16];
    //...
}
Person;

// 使用arrayConstructor并将dynamicArray字段设置为false
staticBytes: arrayConstructor({
    type: DataType.U8Array,
    length: parent.staticBytes.length,
    dynamicArray: false
}),
```

## 函数

`ffi-rs` 支持将js函数指针传递给c函数, 像这样。

```cpp
typedef const void (*FunctionPointer)(int a, bool b, char *c, double d,
                                      char **e, int *f, Person *g);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction\n");

  for (int i = 0; i < 2; i++) {
    int a = 100;
    bool b = false;
    double d = 100.11;
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
    func(a, b, c, d, stringArray, i32Array, p);
  }
}
```

对应上面的代码, 你可以这样使用 `ffi-rs`

```js
const testFunction = () => {
    const func = (a, b, c, d, e, f, g) => {
        equal(a, 100);
        equal(b, false);
        equal(c, "Hello, World!");
        equal(d, "100.11");
        deepStrictEqual(e, ["Hello", "world"]);
        deepStrictEqual(f, [101, 202, 303]);
        deepStrictEqual(g, person);
        logGreen("test function succeed");
        // 当不再使用时释放函数内存
        freePointer({
            paramsType: [funcConstructor({
                paramsType: [
                    DataType.I32,
                    DataType.Boolean,
                    DataType.String,
                    DataType.Double,
                    arrayConstructor({
                        type: DataType.StringArray,
                        length: 2
                    }),
                    arrayConstructor({
                        type: DataType.I32Array,
                        length: 3
                    }),
                    personType,
                ],
                retType: DataType.Void,
            })],
            paramsValue: funcExternal
        })
        if (!process.env.MEMORY) {
            close("libsum");
        }
    };
    // 建议使用createPointer创建函数指针以进行手动内存管理
    const funcExternal = createPointer({
        paramsType: [funcConstructor({
            paramsType: [
                DataType.I32,
                DataType.Boolean,
                DataType.String,
                DataType.Double,
                arrayConstructor({
                    type: DataType.StringArray,
                    length: 2
                }),
                arrayConstructor({
                    type: DataType.I32Array,
                    length: 3
                }),
                personType,
            ],
            retType: DataType.Void,
        })],
        paramsValue: [func]
    })
    load({
        library: "libsum",
        funcName: "callFunction",
        retType: DataType.Void,
        paramsType: [
            DataType.External,
        ],
        paramsValue: unwrapPointer(funcExternal),
    });
}
```

函数参数支持的类型都在上面的示例中

注意, 由于绝大多数情况下开发者将js函数传递给c作为回调, 所以 `ffi-rs` 会从jsfunction创建[threadsafe_function](https://nodejs.org/api/n-api.html#napi_threadsafe_function), 这意味着jsfunction将被异步调用, 并且Node.js进程不会自动退出

## C++

我们将提供更多来自实际场景的示例, 如果你有任何想法, 请提交issue

### 类类型

在C++场景中, 我们可以使用 `DataType.External` 获取类类型指针

在下面的代码中, 我们使用C类型包装C++类型, 如将 `char *` 转换为 `std::string` 并返回类指针

```cpp
MyClass *createMyClass(std::string name, int age) {
  return new MyClass(name, age);
}

extern "C" MyClass *createMyClassFromC(const char *name, int age) {
  return createMyClass(std::string(name), age);
}

extern "C" void printMyClass(MyClass *instance) { instance->print(); }
```

然后, 可以通过以下代码调用

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
freePointer({
    paramsType: [DataType.External],
    paramsValue: [classPointer],
    pointerType: PointerType.CPointer
})
```

## errno

默认情况下, `ffi-rs` 不会输出[errno](https://man7.org/linux/man-pages/man3/errno.3.html)信息, 开发者可以在调用open方法时传递 `errno: true` 来获取它, 像这样

```js
load({
    library: 'libnative',
    funcName: 'setsockopt',
    retType: DataType.I32,
    paramsType: [DataType.I32, DataType.I32, DataType.I32, DataType.External, DataType.I32],
    paramsValue: [socket._handle.fd, level, option, pointer[0], 4],
    errno: true // 将errno设置为true
})

// 上面的代码将返回一个包含三个字段的对象,包括errnoCode、errnoMessage和外部函数返回值
// { errnoCode: 22, errnoMessage: 'Invalid argument (os error 22)', value: -1 }
```

## 内存管理

在单次ffi调用期间释放内存分配很重要, 以防止内存泄漏。

在这个过程中, 哪些类型的数据内存被分配了?

* 在Rust环境中分配在堆上的调用参数, 如`String`
* 在C环境中分配在堆上的返回值, 如`char*`

默认情况下, `ffi-rs` 会释放在Rust中分配的调用参数内存。

但不会释放来自c端的返回值, 因为一些c动态库会自动管理它们的内存(当ffi-rs >= 1.0.79时)

有两种方法可以防止 `ffi-rs` 释放内存

* 调用`load`方法时设置`freeResultMemory: false`, 默认值为false

如果你将freeResultMemory设置为false, `ffi-rs` 将不会释放在c环境中分配的返回结果内存

* 使用`DataType.External`作为paramsType或retType

如果开发者使用 `DataType.External` 作为paramsType或retType, 请在不再使用此内存时使用 `freePointer` 释放指针的内存。参考[test.ts](./test.ts#170)

## runInNewThread

`ffi-rs` 支持在新线程中运行ffi任务, 而不阻塞主线程, 这对于CPU密集型任务很有用。

要使用此功能, 你可以向load方法传递 `runInNewThread` 选项

```js
const testRunInNewThread = async () => {
    // 将返回一个promise,但任务将在新线程中运行
    load({
        library: "libsum",
        funcName: "sum",
        retType: DataType.I32,
        paramsType: [DataType.I32, DataType.I32],
        paramsValue: [1, 2],
        runInNewThread: true,
    }).then(res => {
        equal(res, 3)
    })
}
```
