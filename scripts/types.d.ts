export const enum DataType {
  String = 0,
  I32 = 1,
  Double = 2,
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  Boolean = 6,
  Void = 7,
  I64 = 8,
  U8 = 9,
  U8Array = 10,
  External = 11,
  U64 = 12,
  FloatArray = 13,
  Float = 14,
}

type DataTypeToType<T extends DataType> = T extends DataType.String
  ? string
  : T extends DataType.U8
  ? number
  : T extends DataType.I32
  ? number
  : T extends DataType.I64
  ? number
  : T extends DataType.U64
  ? number
  : T extends DataType.Double
  ? number
  : T extends DataType.External
  ? any
  : T extends DataType.U8Array
  ? number[]
  : T extends DataType.I32Array
  ? number[]
  : T extends DataType.StringArray
  ? string[]
  : T extends DataType.DoubleArray
  ? number[]
  : T extends DataType.FloatArray
  ? number[]
  : T extends DataType.Boolean
  ? boolean
  : T extends DataType.Void
  ? undefined
  : never;


export type ArrayConstructorOptions = {
  type: DataType;
  length: number;
  ffiTypeTag?: string;
  dynamicArray?: boolean
};

export type FuncConstructorOptions = {
  paramsType: Array<FieldType>;
  retType: FieldType;
};

export function arrayConstructor<T extends ArrayConstructorOptions>(
  options: T,
): T;

export function funcConstructor<T extends FuncConstructorOptions>(
  options: T,
): T;

export interface OpenParams {
  library: string;
  path: string;
}
export function open(params: OpenParams): void;
export function close(library: string): void;


export function createPointer(params: {
  paramsType: Array<FieldType>;
  paramsValue: Array<unknown>;
}): unknown[]


export function restorePointer<T extends DataType>(params: {
  retType: Array<FieldType>;
  paramsValue: Array<unknown>;
}): Array<DataTypeToType<T>>

export function unwrapPointer(params: Array<unknown>): Array<unknown>

type ResultWithErrno<T, IncludeErrno extends boolean | undefined = undefined> = IncludeErrno extends true
  ? { value: T; errnoCode: number; errnoMessage: string }
  : T;


export type FieldType =
  | DataType
  | ArrayConstructorOptions
  | FuncConstructorOptions
  | RecordFieldType

interface RecordFieldType extends Record<string, FieldType> { }

type FieldTypeToType<T extends FieldType> = T extends DataType
  ? DataTypeToType<T>
  : T extends ArrayConstructorOptions
  ? DataTypeToType<T['type']>
  : T extends RecordFieldType
  ? { [K in keyof T]: FieldTypeToType<T[K]> }
  : never;


export type FFIParams<T extends FieldType, U extends boolean | undefined = undefined> = {
  library: string;
  funcName: string;
  retType: T;
  paramsType: Array<FieldType>;
  paramsValue: Array<unknown>;
  // whether need output errno
  errno?: U
}
export function load<T extends FieldType, U extends boolean | undefined = undefined>(
  params: FFIParams<T, U>,
): ResultWithErrno<FieldTypeToType<T>, U>

type FuncObj<
  T extends FieldType,
  U extends boolean | undefined = undefined
> = Record<string, Omit<FFIParams<T, U>, 'paramsValue' | 'funcName'>>

export function define<T extends FuncObj<FieldType, boolean | undefined>>(funcs: T): {
  [K in keyof T]: (...paramsValue: Array<unknown>) => ResultWithErrno<FieldTypeToType<T[K]['retType']>, T[K]['errno']>;
}
