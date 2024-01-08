const { execSync } = require('child_process')
const platform = process.platform

const options = {
  stdio: 'inherit'
}

if (platform === 'darwin') {
  execSync('g++ -std=c++11 -dynamiclib -o libsum.so cpp/sum.cpp', options)
}

if (platform === 'linux') {
  execSync('g++ -std=c++11 -fPIC -shared -o libsum.so cpp/sum.cpp', options)
}


if (platform === 'win32') {
  execSync('g++ -std=c++11 -shared -o sum.dll cpp/sum.cpp', options)
  // execSync('g++ -m32 -std=c++11 -shared -o sum32.dll cpp/sum.cpp', options)
}
