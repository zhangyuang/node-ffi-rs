const { load, open, define, close, createPointer, wrapPointer, unwrapPointer, freePointer, DataType, PointerType } = require('../index');

async function main() {
  let dataIdPtr=createPointer({ paramsType: [DataType.String], paramsValue: ['123'] });
  freePointer({
    paramsType: [DataType.String],
    paramsValue: dataIdPtr,
    pointerType: PointerType.CPointer
  })
}
main()

