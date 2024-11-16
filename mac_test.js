const { load, open, DataType } = require("./index");

// 定义 CGRect 结构体
const CGRect = {
  origin: {
    x: DataType.Double,
    y: DataType.Double,
    ffiTypeTag: DataType.StackStruct,
  },
  size: {
    width: DataType.Double,
    height: DataType.Double,
    ffiTypeTag: DataType.StackStruct,
  },
  ffiTypeTag: DataType.StackStruct,
};

// 获取主屏幕尺寸的函数
const getMainDisplaySize = async () => {
  open({
    library: "ApplicationServices",
    path: "/System/Library/Frameworks/ApplicationServices.framework/ApplicationServices",
  });
 
  const bounds = await load({
    library: "ApplicationServices",
    funcName: "CGDisplayBounds",
    paramsType: [DataType.I32], // 不支持 U32 ，使用 I32 报错
    paramsValue: [1],
    retType: CGRect,
  });
  return bounds;
};

if (process.platform === "darwin") {
  getMainDisplaySize().then(console.log);
}
