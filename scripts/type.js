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

    type DataTypeToType<T> = {
      [K in keyof T]: T[K] extends DataType.String ? string :
      T[K] extends DataType.I32 ? number :
      T[K] extends DataType.Double ? number :
      T[K] extends DataType.I32Array ? number[] :
      T[K] extends DataType.StringArray ? string[] :
      T[K] extends DataType.DoubleArray ? number[] :
      T[K] extends DataType.Boolean ? boolean :
      T[K] extends DataType.Void ? undefined :
      never;
    };
    export function load<T extends Record<string, DataType>>(params: FfiParams & { retType: T }): DataTypeToType<T>
    export type ArrayConstructorOptions = {
      type: DataType
      length: number
    }
    export function arrayConstructor<T extends ffiTypeTag>(options: ArrayConstructorOptions): ArrayConstructorOptions & {
      ffiTypeTag: T
    }
    type ffiTypeTag = 'double' | 'string' | 'i32';

    type FfiReturnType<T extends ffiTypeTag> = T extends 'double' ? number :
     T extends 'i32' ? number :
    string;

    export function load<T extends ffiTypeTag>(params: FfiParams & { retType: { ffiTypeTag: T } }): Array<FfiReturnType<T>>
    export type DataFieldType = DataType | Record<string, DataType>| ArrayConstructorOptions & {
      ffiTypeTag: ffiTypeTag
    }
      ${typeContent}
      `)
})()
