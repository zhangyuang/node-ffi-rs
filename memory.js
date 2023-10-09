const Koa = require('koa');
const app = new Koa();
const {unitTest} = require('./test')

app.use(async ctx => {
  console.log((process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2))
  unitTest()
  ctx.body = 'success'
});

app.listen(3000);