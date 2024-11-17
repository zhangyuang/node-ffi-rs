# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-API provides interface (FFI) features for Node.js

<div align="">
<a href="https://github.com/zhangyuang/node-ffi-rs/actions" target="_blank"><img src="https://github.com/zhangyuang/ssr/workflows/CI/badge.svg" alt="githubActions" /></a>
</div>

## Description

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) is a high-performance module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, so ffi-rs was developed to fill that void.

## Features

- High performance ✨
- Better type hints 🧐
- Simpler data description and API interface 💗
- Support more different data types between `Node.js` and `C` 😊
- Support modifying data in place 🥸
- Provide many ways to handle pointer type directly 🐮
- Support running ffi task [in a new thread](#runInNewThread) 🤩️
- Support output [errno](#errno) info 🤔️
- No need to use [ref](https://www.npmjs.com/package/ref) to handle pointer 🤫

## Benchmark

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

## Changelog

See [CHANGELOG.md](./CHANGELOG.md)

## Ecosystem

[abstract-socket-rs](https://github.com/zhangyuang/abstract-socket-rs)

## Install

```bash
$ npm i ffi-rs
```

## Supported Types

Currently, ffi-rs only supports these types of parameters and return values. However, support for more types may be added in the future based on actual usage scenarios.

### Basic Types
- [string](#basic-types)
- [wideString](#basic-types)
- [u8](#basic-types)
- [i32](#basic-types)
- [i64](#basic-types)
- [bigInt](#basic-types)
- [u64](#basic-types)
- [void](#basic-types) (like js undefined)
- [float](#basic-types) (can only be used as paramsType instead of retType)
- [double](#basic-types)
- [boolean](#basic-types)

### Reference Types

- [pointer](#pointer)
- [u8Array](#buffer) (buffer)
- [i32Array](#array)
- [stringArray](#array)
- [doubleArray](#array)
- [floatArray](#array) (can only be used as paramsType instead of retType)
- [object](#struct) (Nested object is also supported in the latest version)
- [function](#function)

### C++ Class

If you want to call a C++ function whose argument type is a class, you can use the `pointer` type. See [tutorial](#C++)

## Supported Platforms

Note: You need to make sure that the compilation environment of the dynamic library is the same as the installation and runtime environment of the `ffi-rs` call.

- darwin-x64
- darwin-arm64
- linux-x64-gnu
- linux-x64-musl
- win32-x64-msvc
- win32-ia32-msvc
- win32-arm64-msvc
- linux-arm64-gnu
- linux-arm64-musl
- linux-arm-gnueabihf

## Usage

View [test.ts](./test.ts) for the latest usage

Here is an example of how to use ffi-rs:

For the following C++ code, we compile this file into a dynamic library

### Write Foreign Function Code

Note: The return value type of a function must be of type C

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

### Compile C Code into a Dynamic Library

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macOS
$ g++ -shared -o libsum.so cpp/sum.cpp # Linux
$ g++ -shared -o sum.dll cpp/sum.cpp # Windows
```

### Call Dynamic Library Using ffi-rs

Then you can use `ffi-rs` to invoke the dynamic library file that contains functions.

### Initialization

It's suggested to develop with TypeScript to get type hints

```js
const { equal } = require('assert')
const { load, DataType, open, close, arrayConstructor, define } = require('ffi-rs')
const a = 1
const b = 100
const dynamicLib = platform === 'win32' ? './sum.dll' : "./libsum.so"
// First open dynamic library with key for close
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
  // freeResultMemory: true, // whether or not need to free the result of return value memory automatically, default is false
})
equal(r, a + b)
// Release library memory when you're not using it.
close('libsum')

// Use define function to define a function signature
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
  }
})
equal(res.sum([1, 2]), 3)
equal(res.atoi(["1000"]), 1000)
```

### Load Main Program Handle

You can also pass an empty path string in the `open` function like [ffi-napi](https://github.com/node-ffi-napi/node-ffi-napi?tab=readme-ov-file#example) to get the main program handle. Refer to [dlopen](https://man7.org/linux/man-pages/man3/dlopen.3.html)

```js
open({
  library: "libnative",
  path: "",
});
// In Darwin/Linux, you can call the atoi function which is included in the basic C library
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

### Basic Types

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

### Buffer

In the latest version, `ffi-rs` supports modifying data in place.

The sample code is as follows

```c
extern int modifyData(char* buffer) {
    // modify buffer data in place
}
```

```js
const arr = Buffer.alloc(200) // create buffer
const res = load({
  library: "libsum",
  funcName: "modifyData",
  retType: DataType.I32,
  paramsType: [
    DataType.U8Array
  ],
  paramsValue: [arr]
})
console.log(arr) // buffer data can be updated
```

### Array

When using `array` as `retType`, you should use `arrayConstructor` to specify the array type with a legal length which is important.

If the length is incorrect, the program may exit abnormally

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

### Pointer

In `ffi-rs`, we use [DataType.External](https://nodejs.org/api/n-api.html#napi_create_external) for wrapping the `pointer` which enables it to be passed between `Node.js` and `C`.

`Pointer` is complicated and underlying, `ffi-rs` provides four functions to handle this pointer including `createPointer`, `restorePointer`, `unwrapPointer`, `wrapPointer`, `freePointer`, `isNullPointer` for different scenes.

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
// get pointer
const ptr = load({
  library: "libsum",
  funcName: "concatenateStrings",
  retType: DataType.External,
  paramsType: [DataType.String, DataType.String],
  paramsValue: [c, d],
})

// send pointer
const string = load({
  library: "libsum",
  funcName: "getStringFromPtr",
  retType: DataType.String,
  paramsType: [DataType.External],
  paramsValue: [ptr],
})
```

#### createPointer

`createPointer` function is used for creating a pointer pointing to a specified type. In order to avoid mistakes, developers have to understand what type this pointer is.

For numeric types like `i32|u8|i64|f64`, createPointer will create a pointer like `*mut i32` pointing to these numbers.

For types that are originally pointer types like `char *` representing `string` type in `C`, createPointer will create a dual pointer like `*mut *mut c_char` pointing to `*mut c_char`. Developers can use `unwrapPointer` to get the internal pointer `*mut c_char`.

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

For the code above, we can use `createPointer` function to wrap a pointer data and send it as paramsValue

```js
const ptrArr: unknown[] = createPointer({
  paramsType: [DataType.DoubleArray],
  paramsValue: [[1.1,2.2]]
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

The two pieces of code above are equivalent

#### restorePointer

Similarly, you can use `restorePointer` to restore data from a `pointer` which is wrapped by `createPointer` or as a return value of a foreign function

```js
const pointerArr = createPointer({
  paramsType: [DataType.DoubleArray],
  paramsValue: [[1.1, 2.2]]
})
const restoreData = restorePointer({
  retType: [arrayConstructor({
    type: DataType.DoubleArray,
    length: 2
  })],
  paramsValue: pointerArr
})
deepStrictEqual(restoreData, [[1.1, 2.2]])
```

#### freePointer

`freePointer` is used to free memory which is not freed automatically.

By default, `ffi-rs` will free data memory for ffi call args and return result to prevent memory leaks. Except in the following cases:

- set `freeResultMemory: false` when calling `load` method

If you set freeResultMemory to false, `ffi-rs` will not release the return result memory which was allocated in the C environment

- Use `DataType.External` as paramsType or retType

If developers use `DataType.External` as paramsType or retType, please use `freePointer` to release the memory of the pointer. ref [test.ts](./test.ts#170)

#### wrapPointer

`wrapPointer` is used to create multiple pointers.

For example, developers can use `wrapPointer` to create a pointer pointing to other existing pointers.

```js
const { wrapPointer } = require('ffi-rs')
// ptr type is *mut c_char
const ptr = load({
  library: "libsum",
  funcName: "concatenateStrings",
  retType: DataType.External,
  paramsType: [DataType.String, DataType.String],
  paramsValue: [c, d],
})

// wrapPtr type is *mut *mut c_char
const wrapPtr = wrapPointer([ptr])[0]
```

#### unwrapPointer

`unwrapPointer` is opposite to `wrapPointer` which is used to get the internal pointer for multiple pointers

```js
const { unwrapPointer, createPointer } = require('ffi-rs')
// ptr type is *mut *mut c_char
let ptr = createPointer({
  paramsType: [DataType.String],
  paramsValue: ["foo"]
})

// unwrapPtr type is *mut c_char
const unwrapPtr = unwrapPointer([ptr])[0]
```

### Struct

To create a C struct or get a C struct as a return type, you need to define the types of the parameters strictly in the order in which the fields of the C structure are defined.

`ffi-rs` provides a C struct named `Person` with many types of fields in [sum.cpp](https://github.com/zhangyuang/node-ffi-rs/blob/master/cpp/sum.cpp#L48)

The example call method about how to call a foreign function to create a `Person` struct or use `Person` struct as a return value is [here](https://github.com/zhangyuang/node-ffi-rs/blob/master/test.ts#L289)

#### Use array in struct

There are two types of arrays in C language like `int* array` and `int array[100]` that have some different usages.

The first type `int* array` is a pointer type storing the first address of the array.

The second type `int array[100]` is a fixed-length array and each element in the array has a continuous address.

If you use an array as a function parameter, this usually passes an array pointer regardless of which type you define. But if the array type is defined in a struct, the two types of array definitions will cause different sizes and alignments of the struct.

So, `ffi-rs` needs to distinguish between the two types.

By default, `ffi-rs` uses pointer arrays to calculate struct. If you confirm there should be a static array, you can define it in this way:

```c
typedef struct Person {
  //...
  uint8_t staticBytes[16];
  //...
} Person;

// use arrayConstructor and set ffiTypeTag field to DataType.StackArray
staticBytes: arrayConstructor({
  type: DataType.U8Array,
  length: parent.staticBytes.length,
  ffiTypeTag: DataType.StackArray
}),
```

## Function

`ffi-rs` supports passing JS function pointers to C functions, like this:

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

Corresponding to the code above, you can use `ffi-rs` like this:

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
    // free function memory when it is not in use
    freePointer({
      paramsType: [funcConstructor({
        paramsType: [
          DataType.I32,
          DataType.Boolean,
          DataType.String,
          DataType.Double,
          arrayConstructor({ type: DataType.StringArray, length: 2 }),
          arrayConstructor({ type: DataType.I32Array, length: 3 }),
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
  // suggest using createPointer to create a function pointer for manual memory management
  const funcExternal = createPointer({
    paramsType: [funcConstructor({
      paramsType: [
        DataType.I32,
        DataType.Boolean,
        DataType.String,
        DataType.Double,
        arrayConstructor({ type: DataType.StringArray, length: 2 }),
        arrayConstructor({ type: DataType.I32Array, length: 3 }),
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

The function parameters support all types in the example above.

Attention: since the vast majority of scenarios developers pass JS functions to C as callbacks, `ffi-rs` will create [threadsafe_function](https://nodejs.org/api/n-api.html#napi_threadsafe_function) from JS functions which means the JS function will be called asynchronously, and the Node.js process will not exit automatically.

## C++

We'll provide more examples from real-world scenarios. If you have any ideas, please submit an issue.

### Class type

In C++ scenarios, we can use `DataType.External` to get a class type pointer.

In the code below, we use C types to wrap C++ types such as converting `char *` to `std::string` and returning a class pointer:

```cpp
MyClass *createMyClass(std::string name, int age) {
  return new MyClass(name, age);
}

extern "C" MyClass *createMyClassFromC(const char *name, int age) {
  return createMyClass(std::string(name), age);
}

extern "C" void printMyClass(MyClass *instance) { instance->print(); }
```

And then, it can be called by the following code:

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

By default, `ffi-rs` will not output [errno](https://man7.org/linux/man-pages/man3/errno.3.html) info. Developers can get it by passing `errno: true` when calling the open method like:

```js
load({
   library: 'libnative',
   funcName: 'setsockopt',
   retType: DataType.I32,
   paramsType: [DataType.I32, DataType.I32, DataType.I32, DataType.External, DataType.I32],
   paramsValue: [socket._handle.fd, level, option, pointer[0], 4],
   errno: true // set errno as true
})

// The above code will return an object including three fields: errnoCode, errnoMessage, and the foreign function return value
// { errnoCode: 22, errnoMessage: 'Invalid argument (os error 22)', value: -1 }
```

## Memory Management

It's important to free the memory allocations during a single ffi call to prevent memory leaks.

What kinds of data memory are allocated in this?

- Call parameters in the Rust environment which are allocated in the heap like `String`
- Return value which in the C environment which are allocated in the heap like `char*`

By default, `ffi-rs` will free call parameters memory which are allocated in Rust.

But it will not free the return value from the C side since some C dynamic libraries will manage their memory automatically (when ffi-rs >= 1.0.79)

There are two ways to prevent `ffi-rs` from releasing memory:

- Set `freeResultMemory: false` when calling `load` method, the default value is false

If you set freeResultMemory to false, `ffi-rs` will not release the return result memory which was allocated in the C environment

- Use `DataType.External` as paramsType or retType

If developers use `DataType.External` as paramsType or retType, please use `freePointer` to release the memory of the pointer when this memory is no longer in use. ref [test.ts](./test.ts#170)

## runInNewThread

`ffi-rs` supports running ffi tasks in a new thread without blocking the main thread, which is useful for CPU-intensive tasks.

To use this feature, you can pass the `runInNewThread` option to the load method:

```js
const testRunInNewThread = async () => {
  // will return a promise but the task will run in a new thread
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