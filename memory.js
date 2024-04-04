const Koa = require('koa');
const app = new Koa();
const { load, RetType, ParamsType } = require('./index')
let smallString = "abc";
let largeString = smallString.repeat(200);

app.use(async ctx => {
  console.log((process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2))
  const res = load({
    library: "./libsum.so",
    funcName: 'concatenateStrings',
    retType: ParamsType.String,
    paramsType: [ParamsType.String, ParamsType.String],
    paramsValue: [largeString, smallString]
  })
  ctx.body = res

});

app.listen(3000);
