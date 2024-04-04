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
    `)

  const typeContent = (await readFile(resolve(process.cwd(), './index.d.ts'))).toString()
    .replace('paramsType: Array<unknown>', 'paramsType: Array<DataFieldType>')
    .replace('retType: unknown', 'retType: DataFieldType')
  await writeFile(resolve(process.cwd(), './index.d.ts'), `
    export function load(params: FfiParams & {
      retType: DataType.String
    }): string
    export function load(params: FfiParams & {
      retType: DataType.I32 | DataType.Double
    }): number

    export function load(params: FfiParams & {
      retType: DataType.Boolean
    }): Boolean

    export function load(params: FfiParams & {
      retType: DataType.Void
    }): undefined

    type DataTypeToType<T extends DataType> =
      T extends DataType.String ? string :
      T extends DataType.I32 ? number :
      T extends DataType.Double ? number :
      T extends DataType.I32Array ? number[] :
      T extends DataType.StringArray ? string[] :
      T extends DataType.DoubleArray ? number[] :
      T extends DataType.Boolean ? boolean :
      T extends DataType.Void ? undefined :
      never


    export type ArrayConstructorOptions = {
      type: DataType
      length: number
      ffiTypeTag?: string
    }
    export function arrayConstructor(options: ArrayConstructorOptions): ArrayConstructorOptions

    export function load<T extends DataType>(params: FfiParams & {
      retType: ArrayConstructorOptions
    }): DataTypeToType<T>

    export type DataFieldType = DataType | Record<string, DataType> | ArrayConstructorOptions

    export function load<T extends Record<string, DataType>>(params: FfiParams & {
      retType: {
        type: T
        length: number
        ffiTypeTag?: string
      }
    }): DataTypeToType<T>
    export type DataFieldType = DataType | Record<string, DataType> | ArrayConstructorOptions
      ${typeContent}
      `)
})()
