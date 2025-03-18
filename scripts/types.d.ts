
interface RawPointer { }

export interface JsExternal {
  _externalDataPlaceholder: RawPointer;
}

export enum DataType {
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
  /**
   As params will be transformed to i64, as return value will be created from i64
  */
  BigInt = 16,
  I16 = 17,
  StructArray = 18,
  I16Array = 19,
  StackStruct = 999, // reserve keyword
  StackArray = 996,
  Function = 998,
  Array = 997,
}

type DataTypeToType<T> = T extends DataType.String
  ? string
  : T extends DataType.WString
  ? string
  : T extends DataType.U8
  ? number
  : T extends DataType.I32
  ? number
  : T extends DataType.I64
  ? number
  : T extends DataType.BigInt
  ? BigInt
  : T extends DataType.U64
  ? number
  : T extends DataType.Double
  ? number
  : T extends DataType.External
  ? JsExternal
  : T extends DataType.U8Array
  ? number[]
  : T extends DataType.I16Array
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

export enum FFITypeTag {
  StackStruct = DataType.StackStruct,
  StackArray = DataType.StackArray,
}

export interface ArrayConstructorOptions {
  type: DataType;
  length: number;
  ffiTypeTag?: FFITypeTag;
  dynamicArray?: boolean
  structItemType?: RecordFieldType
}

export interface FuncConstructorOptions {
  paramsType: FieldType[];
  retType: FieldType;
  needFree?: boolean
  // Default:false, whether or not free function call params memory automatically which are allocated in c side
  freeCFuncParamsMemory?: boolean
}

export function arrayConstructor(options: ArrayConstructorOptions): ArrayConstructorOptions;

export function funcConstructor(options: FuncConstructorOptions): FuncConstructorOptions;

export interface OpenParams {
  library: string;
  path: string;
}

export function open(params: OpenParams): void;
export function close(library: string): void;

export function createPointer(params: {
  paramsType: FieldType[];
  paramsValue: unknown[];
}): JsExternal[]

export enum PointerType {
  RsPointer = 0,
  CPointer = 1
}

export function freePointer(params: {
  paramsType: FieldType[];
  paramsValue: JsExternal[];
  pointerType: PointerType
}): void

export function restorePointer<T>(params: {
  retType: FieldType[];
  paramsValue: JsExternal[];
}): DataTypeToType<T>[]

export function unwrapPointer(params: JsExternal[]): JsExternal[]

export function wrapPointer(params: JsExternal[]): JsExternal[]

export function isNullPointer(params: JsExternal): boolean

type ResultWithErrno<T, E = undefined> = E extends true
  ? { value: T; errnoCode: number; errnoMessage: string }
  : T;

type ResultWithPromise<T, P = undefined> = P extends true
  ? Promise<T>
  : T;

export type FieldType =
  | DataType
  | ArrayConstructorOptions
  | FuncConstructorOptions
  | RecordFieldType

interface RecordFieldType extends Record<string, FieldType> { }

type FieldTypeToType<T> = T extends DataType
  ? DataTypeToType<T>
  : T extends ArrayConstructorOptions
  ? DataTypeToType<T['type']>
  : T extends RecordFieldType
  ? { [K in keyof T]: FieldTypeToType<T[K]> }
  : never;

export interface FFIParams<T, E = undefined, R = undefined> {
  library: string;
  funcName: string;
  retType: T;
  paramsType: FieldType[];
  paramsValue: unknown[];
  // whether need output errno
  errno?: E
  runInNewThread?: R
  // Default:false, whether or not need to free the result of return value memory automatically
  freeResultMemory?: boolean
}

export function load<T extends FieldType, E = undefined, R extends boolean | undefined = undefined>(
  params: FFIParams<T, E, R>
): R extends true ? Promise<ResultWithErrno<FieldTypeToType<T>, E>> : ResultWithErrno<FieldTypeToType<T>, E>

type FuncObj<T, E = undefined, R = undefined> = Record<string, Omit<FFIParams<T, E, R>, 'paramsValue' | 'funcName'>>

export function define<T extends FuncObj<FieldType, boolean | undefined, boolean | undefined>>(funcs: T): {
  [K in keyof T]: (paramsValue: unknown[]) => ResultWithPromise<ResultWithErrno<FieldTypeToType<T[K]['retType']>, T[K]['errno']>, T[K]['runInNewThread']>;
}
