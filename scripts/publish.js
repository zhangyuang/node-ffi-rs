const { resolve } = require('path');
const { cp } = require('shelljs');
const { promises: fsPromises, writeFileSync } = require('fs');
const { execSync } = require('child_process');

const myResolve = dir => resolve(process.cwd(), dir);
const pkg = require(resolve(process.cwd(), './package.json'))
const { version, optionalDependencies } = pkg
for (const key in optionalDependencies) {
  optionalDependencies[key] = version
}
console.log('yuuang', pkg)
writeFileSync(resolve(process.cwd(), './package.json'), JSON.stringify(pkg, null, 2))

const isPreReleaseVersion = () => {
  const preReleaseRegex = /-\w+/;
  return preReleaseRegex.test(version);
};

const publishPackage = (path) => {
  const pkgPath = resolve(path, 'package.json');
  const pkgJson = require(pkgPath);
  const { version } = pkgJson;
  const tag = isPreReleaseVersion(version) ? `--tag alpha` : '';
  execSync(`npm publish ${tag}`, { cwd: path, stdio: 'inherit' });
};

const publishSubpackages = async () => {
  const folders = await fsPromises.readdir(myResolve('./npm'));
  for (const item of folders) {
    if (item !== '.DS_Store') {
      cp(myResolve('./README.md'), myResolve(`./npm/${item}`));
      const p = require(myResolve(`./npm/${item}/package.json`))
      p.version = version
      writeFileSync(myResolve(`./npm/${item}/package.json`), JSON.stringify(p, null, 2))
      publishPackage(myResolve(`./npm/${item}`), item);
    }
  }
};

const publishRoot = async () => {
  const rootPath = process.cwd();
  publishPackage(rootPath);
  pkg.name = 'node-ffi-rs'
  writeFileSync(resolve(process.cwd(), './package.json'), JSON.stringify(pkg, null, 2))
  publishPackage(rootPath);
};

Promise.all([
  publishSubpackages(),
  publishRoot()
]).then(() => {
  console.log('Publish succeeded');
})
  .catch(err => {
    console.error(`Publish failed: ${err}`);
  });
