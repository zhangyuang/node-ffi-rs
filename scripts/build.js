const { execSync } = require("child_process");
const { cp } = require('shelljs')
const { resolve } = require('path')

const cwd = process.cwd()

const options = {
  stdio: "inherit",
};
const target = process.env.target;
execSync(
  `yarn build:c && napi build --platform --release --js-package-name @yuuang/ffi-rs ${target ? `--target ${target}` : ""}`,
  options,
);
cp(resolve(cwd, './scripts/type.js'), resolve(cwd, './index.js'))
cp(resolve(cwd, './scripts/types.d.ts'), resolve(cwd, './index.d.ts'))
