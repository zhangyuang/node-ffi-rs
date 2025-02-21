const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'ffi-rs.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.android-arm64.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'ffi-rs.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.android-arm-eabi.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'ffi-rs.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'ffi-rs.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'ffi-rs.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'ffi-rs.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./ffi-rs.darwin-universal.node')
      } else {
        nativeBinding = require('@yuuang/ffi-rs-darwin-universal')
      }
      break
    } catch { }
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'ffi-rs.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.darwin-x64.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'ffi-rs.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.darwin-arm64.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'ffi-rs.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./ffi-rs.freebsd-x64.node')
      } else {
        nativeBinding = require('@yuuang/ffi-rs-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-x64-musl.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'ffi-rs.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./ffi-rs.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@yuuang/ffi-rs-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'ffi-rs.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./ffi-rs.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@yuuang/ffi-rs-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
    (processParamsTypeForArray(params))
}

const { DataType, createPointer, restorePointer, unwrapPointer, wrapPointer, freePointer, open, close, load, isNullPointer, FFITypeTag } = nativeBinding
DataType.StackStruct = 999
DataType.Function = 998
DataType.Array = 997
DataType.StackArray = 996
exports.DataType = DataType
exports.PointerType = nativeBinding.PointerType
exports.open = open
exports.close = close
exports.load = load
exports.isNullPointer = isNullPointer
exports.FFITypeTag = FFITypeTag
const arrayDataType = [DataType.I16Array, DataType.I32Array, DataType.StringArray, DataType.DoubleArray, DataType.U8Array, DataType.FloatArray]
const arrayConstructor = (options) => ({
  ffiTypeTag: FFITypeTag.Array,
  ...options
})

const processParamsTypeForArray = (params) => {
  params.paramsType = params.paramsType?.map((paramType, index) => {
    if (arrayDataType.includes(paramType)) {
      return arrayConstructor({
        type: paramType,
        length: params.paramsValue[index].length,
      })
    }
    return paramType
  })
  return params
}

const setFreePointerTag = (params) => {
  params.paramsType = params.paramsType?.map((paramType, index) => {
    if (paramType.ffiTypeTag === FFITypeTag.Function) {
      paramType.needFree = true
    }
    return paramType
  })
  return params
}

const wrapLoad = (params) => {
  if (params.freeResultMemory === undefined) {
    params.freeResultMemory = false
  }
  return load(processParamsTypeForArray(params))
}
exports.load = wrapLoad
exports.createPointer = (params) => createPointer(processParamsTypeForArray(params))
exports.restorePointer = (params) => restorePointer(processParamsTypeForArray(params))
exports.unwrapPointer = (params) => unwrapPointer(processParamsTypeForArray(params))
exports.wrapPointer = (params) => wrapPointer(processParamsTypeForArray(params))
exports.freePointer = (params) => freePointer(setFreePointerTag(processParamsTypeForArray(params)))
exports.arrayConstructor = arrayConstructor

exports.funcConstructor = (options) => ({
  ffiTypeTag: FFITypeTag.Function,
  needFree: false,
  freeCFuncParamsMemory: false,
  ...options,
})
exports.define = (obj) => {
  const res = {}
  Object.entries(obj).map(([funcName, funcDesc]) => {
    res[funcName] = (paramsValue = []) => wrapLoad({
      ...funcDesc,
      funcName,
      paramsValue
    })
  })
  return res
}
