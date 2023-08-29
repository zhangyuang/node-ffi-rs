const { readFile, writeFile } = require('fs/promises')
const { resolve } = require('path');

(async () => {
  const typeContent = (await readFile(resolve(process.cwd(), './index.d.ts'))).toString()
  await writeFile(resolve(process.cwd(), './index.d.ts'), `
export type NapiIndexMap = Record<String, ParamsType>
      ${typeContent}
      `)
})()
