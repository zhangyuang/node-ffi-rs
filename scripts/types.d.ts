/* eslint-disable */
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
  : T extends DataType.Boolean
  ? boolean
  : T extends DataType.Void
  ? undefined
  : never;

type DataFieldTypeToType<T extends DataFieldType<DataType>> = T extends DataType
  ? DataTypeToType<T>
  : T extends ArrayConstructorOptions<infer U>
  ? DataTypeToType<U>
  : never;

export type ArrayConstructorOptions<T extends DataType> = {
  type: T;
  length: number;
  ffiTypeTag?: string;
  dynamicArray?: boolean
};

export type FuncConstructorOptions<T extends DataType> = {
  paramsType: Array<DataRecordFieldType<T>>;
  retType: DataFieldType<T>;
};

export function arrayConstructor<T extends DataType>(
  options: ArrayConstructorOptions<T>,
): ArrayConstructorOptions<T>;

export function funcConstructor<T extends DataType>(
  options: FuncConstructorOptions<T>,
): Func;


type Func = <T extends DataType>() => FuncConstructorOptions<T>;

export type DataFieldType<T extends DataType> =
  | DataType
  | Record<string, DataType>
  | ArrayConstructorOptions<T>
  | Func
  | {};

export type DataRecordFieldType<T extends DataType> =
  | Record<string, DataFieldType<T>>
  | DataFieldType<T>
  | {};

export interface FFIParams<T extends DataType> {
  library: string;
  funcName: string;
  retType: DataFieldType<T>;
  paramsType: Array<DataRecordFieldType<T>>;
  paramsValue: Array<unknown>;
  // whether need stdout errno
  errno?: boolean
}
export interface FFIParams<T extends DataType> {
  library: string;
  funcName: string;
  retType: DataFieldType<T>;
  paramsType: Array<DataRecordFieldType<T>>;
  paramsValue: Array<unknown>;
}
export interface OpenParams {
  library: string;
  path: string;
}
export function open(params: OpenParams): void;
export function close(library: string): void;
export function load<T extends DataType>(
  params: Omit<FFIParams<T>, "retType"> & {
    retType: ArrayConstructorOptions<T>;
  },
): DataTypeToType<T>;

export function createPointer<T extends DataType>(params: {
  paramsType: Array<DataRecordFieldType<T>>;
  paramsValue: Array<unknown>;
}): unknown[]


export function restorePointer<T extends DataType>(params: {
  retType: Array<DataRecordFieldType<T>>;
  paramsValue: Array<unknown>;
}): Array<DataTypeToType<T>>

export function unwrapPointer(params: Array<unknown>): Array<unknown>

type ResultWithErrno<T, IncludeErrno extends boolean | undefined> = IncludeErrno extends true
  ? { value: T; errnoCode: number; errnoMessage: string }
  : T;

export function load<
  T extends DataType,
  U extends Record<string, DataFieldType<T>>,
  IncludeErrno extends boolean | undefined = undefined,
>(
  params: Omit<FFIParams<T>, "retType"> & {
    retType?: U;
    errno?: IncludeErrno;
  },
): { [K in keyof U]: ResultWithErrno<DataFieldTypeToType<U[K]>, IncludeErrno> };

export function load<T extends DataType, IncludeErrno extends boolean | undefined = undefined>(
  params: Omit<FFIParams<T>, "retType"> & {
    retType: T;
    errno?: IncludeErrno;
  },
): ResultWithErrno<DataTypeToType<T>, IncludeErrno>;

export function load<T extends DataType, IncludeErrno extends boolean | undefined = undefined>(
  params: Omit<FFIParams<T>, "retType"> & {
    retType: ArrayConstructorOptions<T>;
    errno?: IncludeErrno;
  },
): ResultWithErrno<DataTypeToType<T>, IncludeErrno>;

export function define<
  T extends DataType,
  Obj extends Record<string, Omit<FFIParams<T>, "retType" | 'funcName' | 'paramsValue'> & {
    retType: T;
    errno?: IncludeErrno;
  }>,
  IncludeErrno extends boolean | undefined = undefined
>(
  obj: Obj
): {
    [K in keyof Obj]: (
      p: unknown[]
    ) => any
  };
