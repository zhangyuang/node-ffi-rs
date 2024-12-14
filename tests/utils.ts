export const logGreen = (text) => {
  if (process.env.SILENT) return
  console.log('\x1b[32m%s\x1b[0m', text);
}
