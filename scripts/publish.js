const { resolve } = require('path');
const { cp } = require('shelljs');
const { promises: fsPromises, writeFileSync } = require('fs');
const { execSync } = require('child_process');

const myResolve = dir => resolve(process.cwd(), dir);
const pkg = require(resolve(process.cwd(), './package.json'))
const { version } = pkg

const isPreReleaseVersion = () => {
  const preReleaseRegex = /-\w+/;
  return preReleaseRegex.test(version);
};

const publishPackage = (path) => {
  const tag = isPreReleaseVersion() ? `--tag alpha` : '';
  execSync(`npm publish ${tag}`, { cwd: path, stdio: 'inherit' });
};
const optionalDependencies = {}
const publishSubpackages = async () => {
  const folders = await fsPromises.readdir(myResolve('./npm'));
  for (const item of folders) {
    if (item !== '.DS_Store') {
      cp(myResolve('./README.md'), myResolve(`./npm/${item}`));
      const p = require(myResolve(`./npm/${item}/package.json`))
      p.version = version
      optionalDependencies[p.name] = version
      await fsPromises.writeFile(myResolve(`./npm/${item}/package.json`), JSON.stringify(p, null, 2))
      publishPackage(myResolve(`./npm/${item}`), item);
    }
  }
  await fsPromises.writeFile(resolve(process.cwd(), './package.json'), JSON.stringify(pkg, null, 2))
  console.log('yuuang', pkg)
};

const publishRoot = async () => {
  const rootPath = process.cwd();
  publishPackage(rootPath);
  pkg.name = 'node-ffi-rs'
  writeFileSync(resolve(process.cwd(), './package.json'), JSON.stringify(pkg, null, 2))
  publishPackage(rootPath);
};

publishSubpackages().then(async () => {
  await publishRoot()
}).
  then(() => {
    console.log('Publish succeeded');
  })
  .catch(err => {
    console.error(`Publish failed: ${err}`);
    procee.exit(1)
  });
