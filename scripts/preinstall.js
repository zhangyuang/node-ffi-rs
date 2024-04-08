const { writeFileSync } = require('fs')
const { resolve } = require('path')

const pkgJson = require(resolve(process.cwd(), './package.json'))

delete pkgJson['optionalDependencies']


writeFileSync(resolve(process.cwd(), './package.json'), JSON.stringify(pkgJson, "", "\n"))
