const { execSync } = require("child_process");

const options = {
  stdio: "inherit",
};
const target = process.env.target;
console.log('xxx', target)
execSync(
  `yarn build:c && napi build --platform --release --js-package-name @yuuang/ffi-rs ${target ? `--target ${target}` : ""
  }&& node scripts/type.js`,
  options,
);
