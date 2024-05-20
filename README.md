# ffi-rs

<div>
<a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README.md">English</a> | <a href="https://github.com/zhangyuang/node-ffi-rs/blob/master/README_Zh.md">简体中文</a>
</div>

A module written in Rust and N-API provides interface (FFI) features for Node.js

<div align="">
<a href="https://github.com/zhangyuang/node-ffi-rs/actions" target="_blank"><img src="https://github.com/zhangyuang/ssr/workflows/CI/badge.svg" alt="githubActions" />
</div>

## Description

[ffi-rs](https://github.com/zhangyuang/node-ffi-rs) is a high-performance module written in Rust and N-API that provides FFI (Foreign Function Interface) features for Node.js. It allows developers to call functions written in other languages such as C++, C, and Rust directly from JavaScript without writing any C++ code.

This module aims to provide similar functionality to the node-ffi module but with a completely rewritten underlying codebase. The node-ffi module has been unmaintained for several years and is no longer usable, so ffi-rs was developed to fill that void.

## features

- High performance ✨
- Better type hints 🧐
- Simpler data description and API interface 💗
- Support more different data types between `Node.js` and `c` 😊
- Support modify data in place 🥸
- Provide many ways to handle pointer type directly 🐮
- Support run ffi task [in a new thread](#runInNewThread) 🤩️
- Support output [errno](#errno) info 🤔️
- No need to use [ref](https://www.npmjs.com/package/ref) to handle pointer 🤫

## benchmark

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

## changelog

See [CHANGELOG.md](./CHANGELOG.md)

## ecosystem

[abstract-socket-rs](https://github.com/zhangyuang/abstract-socket-rs)

## install

```js
$ npm i ffi-rs
```

## Support type

Currently, ffi-rs only supports these types of parameters and return values. However, support for more types may be added in the future based on actual usage scenarios.

### Basic Type
- [string](#basic-types)
- [u8](#basic-types)
- [i32](#basic-types)
- [i64](#basic-types)
- [u64](#basic-types)
- [void](#basic-types)(like js undefined)
- [float](#basic-types)(can only be used as paramsType instead of retType)
- [double](#basic-types)
- [boolean](#basic-types)

### Reference Type

- [pointer](#pointer)
- [u8Array](#buffer)(buffer)
- [i32Array](#array)
- [stringArray](#array)
- [doubleArray](#array)
- [floatArray](#array)(can only be used as paramsType instead of retType)
- [object](#struct)(Nested object is also supported at the latest version)
- [function](#function)

### C++ Class

If you want to call C++ function whose argument type is class, you can use `pointer` type, see [tutorial](#C++)

## Support Platform

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

## Usage

View [test.ts](./test.ts) get the latest usage

Here is an example of how to use ffi-rs:

For the following C++ code, we compile this file into a dynamic library

### write foreign function code

Note: The return value type of a function must be of type c

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

### compile C code into a dynamic library

```bash
$ g++ -dynamiclib -o libsum.so cpp/sum.cpp # macos
$ g++ -shared -o libsum.so cpp/sum.cpp # linux
$ g++ -shared -o sum.dll cpp/sum.cpp # win
```

### call dynamic library by ffi-rs

Then you can use `ffi-rs` to invoke the dynamic library file that contains functions.

### Initialization

Suggested develop with typescript to get type hints

```js
const { equal } = require('assert')
const { load, DataType, open, close, arrayConstructor, define } = require('ffi-rs')
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

// use define function to define a function signature
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

### Load Main Program handle

You can alse pass emptry path string in `open` function like [ffi-napi](https://github.com/node-ffi-napi/node-ffi-napi?tab=readme-ov-file#example) to get the main program handle refer [dlopen](https://man7.org/linux/man-pages/man3/dlopen.3.html)

```js
open({
  library: "libnative",
  path: "",
});
// In darwin/linux, you can call atoi function which is included in the basic c library
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

`Pointer` is complicated and underlying, `ffi-rs` provide four functions to handle this pointer include `createPointer`, `restorePointer`, `unwrapPointer`, `freePointer` for different scene.

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

`createPointer` function is used for create a pointer point to specify type. In order to avoid mistaks, developers have to understand what type this pointer is.

For numeric type like `i32|u8|i64|f64`, createPointer will create a pointer like `*mut i32` point to there number

For types that are originally pointer types like `char *` represent `string` type in `c`, createPointer will create a dual pointer like `*mut *mut c_char` point to `*mut c_char`.Developers can use `unwrapPointer` get the interal pointer `*mut c_char`

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

Similarly, you can use `restorePointer` to restore data from `pointer` which is wrapped by `createPointer` or as a return value of foreign function

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

`freePointer` is used to free memory which are not be freed automatically.

At default, `ffi-rs` will free data memory for ffi call args and return result prevent memory leak.Except in the following cases.

- set `needFreeResultMemory: false` when call `load` method

If you set needFreeResultMemory to false, `ffi-rs` will not release the return result memory which was malloc in c environment

- Use `DataType.External` as paramsType or retType

If developers use `DataType.External` as paramsType or retType, please use `freePointer` to release the memory of pointer. ref [test.ts](./test.ts#170)

#### unwrapPointer

`unwrapPointer` is oppsite to `wrapPointer` which is used to get the internal pointer for multiple pointer

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

To create a c struct or get a c struct as a return type, you need to define the types of the parameters strictly in the order in which the fields of the c structure are defined.

`ffi-rs` provide a c struct named `Person` with many types of field in [sum.cpp](https://github.com/zhangyuang/node-ffi-rs/blob/master/cpp/sum.cpp#L48)

The example call method about how to call foreign function to create `Person` struct or use `Person` struct as a return value is [here](https://github.com/zhangyuang/node-ffi-rs/blob/master/test.ts#L289)

#### Use array in struct

There are two types of array in c language like `int* array` and `int array[100]` yhat have some different usages.

The first type `int* array` is a pointer type store the first address of the array.

The second type `int array[100]` is a fixed length array and each element in array has continous address.

If you use a array as function parameter, this usually passes an array pointer regardless of which type you define.But if the array type is defined in struct, the two types of array define will cause different size and align of struct.

So, `ffi-rs` need to distinguish between the two types.

By default, `ffi-rs` use pointer array to calculate struct. If you confirm there should use static array, you can define it in the way

```js
typedef struct Person {
  //...
  uint8_t staticBytes[16];
  //...
} Person;

// use arrayConstructor and set dynamicArray field to false
staticBytes: arrayConstructor({
  type: DataType.U8Array,
  length: parent.staticBytes.length,
  dynamicArray: false
}),
```

## Function

`ffi-rs` supports passing js function pointer to c function, like this.

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

Corresponds to the code above，you can use `ffi-rs` like

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
    // free function memory when it not in use
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
  // suggest use createPointer to create a function pointer for manual memory management
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

The function parameters supports type are all in the example above

Attention，since the vast majority of scenarios developers pass js function to c as a callback, so `ffi-rs` will create [threadsafe_function](https://nodejs.org/api/n-api.html#napi_threadsafe_function) from jsfunction which means the jsfunction will be called asynchronous, and Node.js process will not be exited automatically


## C++

We'll provide more examples from real-world scenarios, if you have any ideas, please submit an issue

### class type

In C++ scene, we can use `DataType.External` to get a class type pointer

In the code below, we use C types to wrap C++ types such as convert `char *` to `std::string` and return class pointer

```cpp
MyClass *createMyClass(std::string name, int age) {
  return new MyClass(name, age);
}

extern "C" MyClass *createMyClassFromC(const char *name, int age) {
  return createMyClass(std::string(name), age);
}

extern "C" void printMyClass(MyClass *instance) { instance->print(); }
```

And then, it can called by the following code

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

By default, `ffi-rs` will not output [errno](https://man7.org/linux/man-pages/man3/errno.3.html) info, developers can get it by pass `errno: true` when call open method like

```js
load({
   library: 'libnative',
   funcName: 'setsockopt',
   retType: DataType.I32,
   paramsType: [DataType.I32, DataType.I32, DataType.I32, DataType.External, DataType.I32],
   paramsValue: [socket._handle.fd, level, option, pointer[0], 4],
   errno: true // set errno as true
})

// The above code will return a object include three fields include errnoCode, errnoMessage, and the foreign function return value
// { errnoCode: 22, errnoMessage: 'Invalid argument (os error 22)', value: -1 }
```

## runInNewThread

`ffi-rs` support run ffi task in a new thread without blocking the main thread which is useful for cpu intensive task.

To use the feature, you can pass `runInNewThread` option to load method

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
