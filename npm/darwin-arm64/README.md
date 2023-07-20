# 在 Node.js 不同进程间共享内存

`sharedmemory-rs` 是一个由 `Rust + Napi` 开发，可以在 `Node.js` 不同进程中共享 `String|Object|Function` 内存的模块。

## 支持的功能

🚀 表示已经实现的功能

| 里程碑                                                                                                                                                                                                                                          | 状态 |
| -------------------------------------------------------------------------------------------------------------------------------------- | ---- |
|  支持回收内存    | 🚀    |
|  支持跨进程共享字符串    | 🚀    |
|  支持跨进程共享 JsObject    | 开发中    |



## 如何使用

```js
// parent.js
const { fork } = require('child_process')

const sharedMemory = require('sharedmemory-rs')

const stringLink = "string.link" // 设置一个内存id memoryId

sharedMemory.init() // 初始化 sharedMemory
sharedMemory.setString(stringLink, "shared String") // 使用该内存 id 存储需要共享的字符串
console.log('Read shared string in parent process', sharedMemory.getString(stringLink))
const child = fork('./child')

child.send('ready')
child.on('message', msg => {
  if (msg === 'finish') {
    sharedMemory.clear(stringLink) // 当不需要使用后记得在主进程销毁该内存块
  }
})

// child.js

const sharedMemory = require('sharedmemory-rs')
process.on("message", msg => {
  if (msg === "ready") {
    console.log('Read shared string in child process', sharedMemory.getString("string.link"))
    process.send("finish")
    process.exit()
  }
})

```
# shared memory for Node.js Process by Rust Napi

`sharedmemory-rs` is a module developed using Rust + Napi that allows sharing String|Object|Function memory between different processes in Node.js.

## Features Implemented

🚀 represent features which has been implemented

| 里程碑                                                                                                                                                                                                                                          | 状态 |
| -------------------------------------------------------------------------------------------------------------------------------------- | ---- |
|  Support memory recycling	    | 🚀    |
|  Support sharing strings across processes	    | 🚀    |
|  Support sharing JsObjects across processes (under development)    | In progress    |


# How to use

```js
// parent.js
const { fork } = require('child_process')

const sharedMemory = require('sharedMemory')

const stringLink = "string.link" // Set a memory id memoryId

sharedMemory.init() // Initialize sharedMemory
sharedMemory.setString(stringLink, "shared String") // Store the string to be shared using the memory id
console.log('Read shared string in parent process', sharedMemory.getString(stringLink))
const child = fork('./child')

child.send('ready')
child.on('message', msg => {
  if (msg === 'finish') {
    sharedMemory.clear(stringLink) // Remember to destroy the memory block in the main process when it is no longer needed
  }
})

// child.js

const sharedMemory = require('sharedMemory')
process.on("message", msg => {
  if (msg === "ready") {
    console.log('Read shared string in child process', sharedMemory.getString("string.link"))
    process.send("finish")
    process.exit()
  }
})
```
