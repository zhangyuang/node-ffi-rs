const koa = require('koa')
const { fork } = require('child_process')
const app = new koa();

const sharedMemory = require('./index')

const stringLink = "string.link"

sharedMemory.init()
sharedMemory.setString("string.link", "parent")
console.log('Read shared string in parent process:', sharedMemory.getString(stringLink))
const child = fork('./child')

// child.send('ready')
// child.on('message', msg => {
//   if (msg === 'finish') {
//     console.log('Read new string in parent process:', sharedMemory.getString(stringLink))
//     sharedMemory.clear(stringLink)
//   }
// })


function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 100; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}

app.use(async (ctx) => {
  const str = generateBigString()
  // sharedMemory.setString('string.link', str)
  // const data = sharedMemory.getString('string.link')
  ctx.body = str
  // sharedMemory.clear('string.link')
});
app.listen(3000)