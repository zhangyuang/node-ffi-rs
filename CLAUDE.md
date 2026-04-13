# node-ffi-rs (ffi-rs)

High-performance FFI module for Node.js, written in Rust + N-API. Allows JS/TS to call C/C++ dynamic libraries (.so/.dylib/.dll) without writing native binding code.

## Quick Reference

```bash
# Install dependencies
pnpm install

# Build Rust native module + compile test C++ library
npm run build        # runs napi build + copies type files
npm run build:c      # compiles cpp/sum.cpp -> libsum.so (or sum.dll)
npm run build:dev    # development build

# Run tests
npm test             # runs: esno ./tests/index.ts

# TypeScript runner: project uses esno (not ts-node)
npx esno tests/some_test.ts
```

## Project Structure

```
index.js              # JS entry point — loads platform-specific .node binary, wraps core functions
index.d.ts            # TypeScript type definitions (public API)
src/
  lib.rs              # Rust core: open, close, load, createPointer exports
  define.rs           # Rust types: DataType enum, RsArgsValue, FFIParams, FFIARRARYDESC
  datatype/
    function.rs       # Callback/closure handling: get_rs_value_from_pointer for callback args
    string.rs         # String marshalling (char* <-> JS string)
    array.rs          # Array type conversion
    buffer.rs         # Buffer/U8Array handling
    pointer.rs        # Pointer create/restore/free
    create_struct.rs  # Struct creation for FFI
    restore_struct.rs # Struct reading from FFI
  utils/
    dataprocess.rs    # Core data marshalling: JS <-> C argument conversion, closure creation
    pointer.rs        # Pointer utilities
    array.rs          # Array utilities
    js_value.rs       # JS value helpers
cpp/
  sum.cpp             # Test C++ library source (all test functions)
  sum.h               # Header for test library
tests/
  index.ts            # Main test file — exercises all features
  types.ts            # Struct type definitions and test data (Person, Parent, etc.)
  struct.ts           # Struct-specific tests
  utils.ts            # Test utilities (logGreen, etc.)
scripts/
  compile.js          # Compiles cpp/sum.cpp to platform-specific shared lib
  build.js            # Full build: build:c + napi build + copy type files
```

## Core API Pattern

### Lifecycle: open -> load -> close

```ts
import { open, load, close, DataType } from "ffi-rs";

// 1. Register library with a key name
open({ library: "mylib", path: "./libmylib.dylib" });
// Pass path="" to open main program handle (access libc functions like atoi)

// 2. Call functions
const result = load({
  library: "mylib",
  funcName: "my_func",
  retType: DataType.I32,
  paramsType: [DataType.I32, DataType.String],
  paramsValue: [42, "hello"],
});

// 3. Release
close("mylib");
```

### DataType Enum (key types)

| DataType | C Type | JS Type |
|----------|--------|---------|
| String (0) | `const char*` | string |
| WString (15) | `wchar_t*` | string |
| I32 (1) | `int32_t` | number |
| I64 (8) | `int64_t` | number |
| U8 (9) | `uint8_t` | number |
| U32 (20) | `uint32_t` | number |
| U64 (12) | `uint64_t` | number |
| Double (2) | `double` | number |
| Float (14) | `float` | number |
| Boolean (6) | `bool` | boolean |
| Void (7) | `void` | undefined |
| U8Array (10) | `uint8_t*` | Buffer |
| I32Array (3) | `int32_t*` | number[] |
| StringArray (4) | `char**` | string[] |
| DoubleArray (5) | `double*` | number[] |
| FloatArray (13) | `float*` | number[] |
| External (11) | `void*` | JsExternal (opaque pointer) |
| BigInt (16) | `int64_t` | BigInt |

### Callback (Function Pointer) Pattern

For passing a JS function as a C function pointer:

```ts
import { funcConstructor, arrayConstructor, createPointer, unwrapPointer, freePointer, PointerType } from "ffi-rs";

// 1. Describe callback signature with funcConstructor
const cbDesc = funcConstructor({
  paramsType: [DataType.I32, DataType.String],
  retType: DataType.Void,
});

// 2. Create function pointer from JS function
const cbExternal = createPointer({
  paramsType: [cbDesc],
  paramsValue: [(a, b) => { console.log(a, b); }],
});

// 3. Pass to C as DataType.External, unwrap one level of indirection
load({
  library: "mylib",
  funcName: "register_callback",
  retType: DataType.Void,
  paramsType: [DataType.External],
  paramsValue: unwrapPointer(cbExternal),
  runInNewThread: true,  // IMPORTANT: required for callbacks to work properly
});

// 4. Free when done
freePointer({ paramsType: [cbDesc], paramsValue: cbExternal, pointerType: PointerType.RsPointer });
```

Key rules:
- `runInNewThread: true` is required when the C side invokes callbacks synchronously, otherwise the JS event loop cannot respond.
- For binary data in callbacks, use `arrayConstructor({ type: DataType.U8Array, length: N })` instead of `DataType.String`.
- `freeCFuncParamsMemory: true` on funcConstructor makes ffi-rs auto-free C-allocated callback parameters.

### Pointer Management

```ts
createPointer({ paramsType, paramsValue })  // Allocate, returns JsExternal[]
unwrapPointer(externals)                     // Dereference one level (pointer-to-pointer -> pointer)
wrapPointer(externals)                       // Add one level of indirection
restorePointer({ retType, paramsValue })     // Read value back to JS
freePointer({ paramsType, paramsValue, pointerType })  // Free memory
isNullPointer(external)                      // Null check
```

### Struct Types

Structs are described as `Record<string, FieldType>` objects:

```ts
const personType = {
  name: DataType.String,
  age: DataType.I32,
  scores: arrayConstructor({ type: DataType.I32Array, length: 3 }),
};
```

### define() Convenience API

Pre-bind function signatures:

```ts
const lib = define({
  sum: { library: "mylib", retType: DataType.I32, paramsType: [DataType.I32, DataType.I32] },
});
lib.sum([1, 2]); // => 3
```

## Build System

- Native module built with `@napi-rs/cli` (napi build)
- Platform-specific binaries published as optional deps: `@yuuang/ffi-rs-{platform}-{arch}`
- `index.js` and `index.d.ts` are generated from `scripts/type.js` and `scripts/types.d.ts` during build
- Test C++ library compiled via `scripts/compile.js` (g++ with platform-specific flags)

## Key Implementation Details

- Rust entry points are in `src/lib.rs` — `open` stores libraries in a global `HashMap`, `load` resolves symbols lazily and caches them
- Callback closures use `libffi::middle::Closure` + N-API `ThreadsafeFunction` for cross-thread JS invocation (see `src/utils/dataprocess.rs`)
- Active closures are tracked in a global `CLOSURE_MAP` for lifecycle management
- `freeResultMemory` defaults to `false` in `index.js` — caller is responsible for memory management by default
