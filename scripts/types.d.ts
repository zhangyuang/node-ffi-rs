
interface RawPointer { }

export interface JsExternal {
  _externalDataPlaceholder: RawPointer;
}

export const enum DataType {
  /**
   UTF-16 String, equivalent to char*
  */
  String = 0,
  /**
   UTF-16 String, equivalent to wchar*
  */
  WString = 15,
  I32 = 1,
  Double = 2,
  /**
    * denotes ordinary i32 number array instead of Uint32Array
  */
  I32Array = 3,
  StringArray = 4,
  /**
    denotes f64 array
  */
  DoubleArray = 5,
  Boolean = 6,
  /**
    equal with undefined
  */
  Void = 7,
  I64 = 8,
  U8 = 9,
  /**
    Equivanlent to Buffer
  */
  U8Array = 10,
  /**
    A type that wrap native pointer
  */
  External = 11,
  U64 = 12,
  FloatArray = 13,
  Float = 14,

  StackStruct = 999 // reserve keyword
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
  ? JsExternal
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
  needFree?: boolean
  // Default:false, whether or not free function call params memory automatically which are allocated in c side
  freeCFuncParamsMemory?: boolean
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
}): JsExternal[]

export enum PointerType {
  RsPointer = 0,
  CPointer = 1
}

export function freePointer(params: {
  paramsType: Array<FieldType>;
  paramsValue: Array<JsExternal>;
  pointerType: PointerType
}): void


export function restorePointer<T extends DataType>(params: {
  retType: Array<FieldType>;
  paramsValue: Array<JsExternal>;
}): Array<DataTypeToType<T>>

export function unwrapPointer(params: Array<JsExternal>): Array<JsExternal>

export function wrapPointer(params: Array<JsExternal>): Array<JsExternal>

type ResultWithErrno<T, IncludeErrno extends boolean | undefined = undefined> = IncludeErrno extends true
  ? { value: T; errnoCode: number; errnoMessage: string }
  : T;

type ResultWithPromise<T, U extends boolean | undefined = undefined> = U extends true
  ? Promise<T>
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


export type FFIParams<T extends FieldType, U extends boolean | undefined = undefined, RunInNewThread extends boolean | undefined = undefined> = {
  library: string;
  funcName: string;
  retType: T;
  paramsType: Array<FieldType>;
  paramsValue: Array<unknown>;
  // whether need output errno
  errno?: U
  runInNewThread?: RunInNewThread
  // Default:false, whether or not need to free the result of return value memory automatically
  freeResultMemory?: boolean
}
export function load<T extends FieldType, U extends boolean | undefined = undefined, RunInNewThread extends boolean | undefined = undefined>(
  params: FFIParams<T, U, RunInNewThread>,
): ResultWithPromise<ResultWithErrno<FieldTypeToType<T>, U>, RunInNewThread>

type FuncObj<
  T extends FieldType,
  U extends boolean | undefined = undefined,
  RunInNewThread extends boolean | undefined = undefined
> = Record<string, Omit<FFIParams<T, U, RunInNewThread>, 'paramsValue' | 'funcName'>>

export function define<T extends FuncObj<FieldType, boolean | undefined>>(funcs: T): {
  [K in keyof T]: (...paramsValue: Array<unknown>) => ResultWithPromise<ResultWithErrno<FieldTypeToType<T[K]['retType']>, T[K]['errno']>, T[K]['runInNewThread']>;
}
