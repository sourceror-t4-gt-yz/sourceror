// import * as check from 'wasm-check';
const check = require('wasm-check');

console.log(`WASM Support: ${check.support()}`);
console.log(`Features:`);
console.log({...check.feature});
