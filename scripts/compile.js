const { execSync } = require('child_process')
const platform = process.platform

const options = {
  stdio: 'inherit'
}

if (platform === 'darwin') {
  execSync('g++ -dynamiclib -o libsum.so cpp/sum.cpp', options)
}

if (platform === 'linux') {
  execSync('g++ -shared -o libsum.so cpp/sum.cpp', options)
}


if (platform === 'win32') {
  execSync('g++ -shared -o sum.dll cpp/sum.cpp', options)
}
