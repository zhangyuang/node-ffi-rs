/* tslint:disable */
/* eslint-disable */

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

type DataFieldTypeToType<T extends DataFieldType<DataType>> =
  T extends DataType ? DataTypeToType<T> :
  T extends ArrayConstructorOptions<infer U> ? DataTypeToType<U> :
  never;

export function load<T extends DataType, U extends Record<string, DataFieldType<T>>>(params: Omit<FfiParams<T>, 'retType'> & {
  retType?: U
}): { [K in keyof U]: DataFieldTypeToType<U[K]> }

export function load<T extends DataType>(params: Omit<FfiParams<T>, 'retType'> & {
  retType: T
}): DataTypeToType<T>

export type ArrayConstructorOptions<T extends DataType> = {
  type: T
  length: number
  ffiTypeTag?: string
}

export type FuncConstructorOptions<T extends DataType> = {
  paramsType: Array<DataFieldType<T>>
}

export function arrayConstructor<T extends DataType>(options: ArrayConstructorOptions<T>): ArrayConstructorOptions<T>

export function funcConstructor<T extends DataType>(options: FuncConstructorOptions<T>): Func

export function load<T extends DataType>(params: Omit<FfiParams<T>, 'retType'> & {
  retType: ArrayConstructorOptions<T>
}): DataTypeToType<T>

type Func = <T extends DataType>() => FuncConstructorOptions<T>

export function load<T extends DataType>(params: Omit<FfiParams<T>, 'retType'> & {
  retType: ArrayConstructorOptions<T>
}): DataTypeToType<T>

export type DataFieldType<T extends DataType> = DataType | Record<string, DataType> | ArrayConstructorOptions<T> | Func


export const enum DataType {
  String = 0,
  I32 = 1,
  Double = 2,
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  Boolean = 6,
  Void = 7
}
export interface FfiParams<T extends DataType> {
  library: string
  funcName: string
  retType: DataFieldType<T>
  paramsType: Array<DataFieldType<T>>
  paramsValue: Array<unknown>
}
export interface OpenParams {
  library: string
  path: string
}
export function open(params: OpenParams): void
export function close(library: string): void
