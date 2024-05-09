const { resolve } = require('path');
const { cp } = require('shelljs');
const { promises: fsPromises } = require('fs');
const { execSync } = require('child_process');

const myResolve = dir => resolve(process.cwd(), dir);

const isPreReleaseVersion = version => {
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
      publishPackage(myResolve(`./npm/${item}`), item);
    }
  }
};

const publishRoot = async () => {
  const rootPath = process.cwd();
  publishPackage(rootPath, 'ffi-rs');
  publishPackage(rootPath, 'node-ffi-rs');
};

publishSubpackages()
  .then(async () => {
    await publishRoot();
  })
  .then(() => {
    console.log('Publish succeeded');
  })
  .catch(err => {
    console.error(`Publish failed: ${err}`);
  });
