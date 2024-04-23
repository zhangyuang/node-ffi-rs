const { readFile, writeFile } = require("fs/promises");
const { resolve } = require("path");

(async () => {
  const entryContent = (await readFile(resolve(process.cwd(), "./index.js")))
    .toString()
    .replace("paramsType: Array<unknown>", "paramsType: Array<DataFieldType>")
    .replace("retType: unknown", "retType: DataFieldType");
  await writeFile(
    resolve(process.cwd(), "./index.js"),
    `
    ${entryContent}
    exports.arrayConstructor = (options) => ({
      dynamicArray: true,
      ...options,
      ffiTypeTag: 'array'
    })
    exports.funcConstructor = (options) => ({
      ffiTypeTag: 'function',
      ...options,
    })
    exports.define = (obj) => {
      const res = {}
      Object.entries(obj).map(([funcName, funcDesc]) => {
        res[funcName] = (paramsValue) => load({
          ...obj[funcName],
          funcName,
          paramsValue
        })
      })
      return res
    }
    `,
  );
  const typesContent = (
    await readFile(resolve(process.cwd(), "./scripts/types.d.ts"))
  ).toString();
  await writeFile(resolve(process.cwd(), "./index.d.ts"), typesContent);
})();
