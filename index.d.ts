/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum RetType {
  String = 0,
  I32 = 1
}
export const enum ParamsType {
  String = 0,
  I32 = 1
}
export interface FfiParams {
  library: string
  funcName: string
  retType: RetType
  paramsType: Array<ParamsType>
  paramsValue: Array<unknown>
}
export function load(params: FfiParams): string | number
