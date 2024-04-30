const { resolve } =require('path')
const { cp } = require('shelljs')
const { promises,  } = require('fs')

const myResolve = dir => resolve(process.cwd(), dir)
const p = async () => {
  const folders = await promises.readdir(myResolve('./npm'))
  for (const item of folders) {
    if (item!=='.DS_Store') {
      cp(myResolve('./README.md'), myResolve(`./npm/${item}`))
    }
  }
}

p().then()