const { readFile, writeFile } = require('fs/promises')
const { resolve } = require('path');

(async () => {
  const entryContent = (await readFile(resolve(process.cwd(), './index.js'))).toString()
    .replace('paramsType: Array<unknown>', 'paramsType: Array<DataFieldType>')
    .replace('retType: unknown', 'retType: DataFieldType')
  await writeFile(resolve(process.cwd(), './index.js'), `
    ${entryContent}
    module.exports.arrayConstructor = (options) => ({
      ...options,
      ffiTypeTag: 'array'
    })
    module.exports.funcConstructor = (options) => (() => ({
      permanent: false,
      ffiTypeTag: 'function',
      uuid: require('uuid').v4(),
      ...options,
    }))
    `)
  const typesContent = (await readFile(resolve(process.cwd(), './scripts/types.d.ts'))).toString()
  await writeFile(resolve(process.cwd(), './index.d.ts'), typesContent)
})()
