const { resolve } = require('path')
const { cp } = require('shelljs')
const { promises } = require('fs')
const { execSync } = require('child_process')

const myResolve = dir => resolve(process.cwd(), dir)
const p = async () => {
  const folders = await promises.readdir(myResolve('./npm'))
  for (const item of folders) {
    if (item !== '.DS_Store') {
      cp(myResolve('./README.md'), myResolve(`./npm/${item}`))
    }
  }
  const pkgJson = require(resolve(process.cwd(), './package.json'))
  pkgJson['name'] = '@yuuang/ffi-rs'
  await promises.writeFile(resolve(process.cwd(), './package.json'), JSON.stringify(pkgJson, null, 2))
}

const main = async () => {
  const pkgJson = require(resolve(process.cwd(), './package.json'))
  pkgJson['name'] = 'ffi-rs'
  await promises.writeFile(resolve(process.cwd(), './package.json'), JSON.stringify(pkgJson, null, 2))
  execSync('npm publish', { stdio: 'inherit' })
  pkgJson['name'] = 'node-ffi-rs'
  await promises.writeFile(resolve(process.cwd(), './package.json'), JSON.stringify(pkgJson, null, 2))
  execSync('npm publish', { stdio: 'inherit' })
}

if (process.env.OPTIONAL) {
  p().then()
}

if (process.env.MAIN) {
  main().then()
}
