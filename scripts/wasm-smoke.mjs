// Runs the wasm_smoke example: node scripts/wasm-smoke.mjs <wasm_smoke.wasm>
// The module must import nothing (libghostty-vt is freestanding) and put the cursor at 2,1.
import { readFileSync } from "node:fs";

const module = await WebAssembly.compile(readFileSync(process.argv[2]));
const imports = WebAssembly.Module.imports(module);
if (imports.length > 0) {
  console.error("unexpected imports:", imports);
  process.exit(1);
}
const instance = await WebAssembly.instantiate(module, {});
const result = instance.exports.smoke();
console.log("smoke:", result);
process.exit(result === 201 ? 0 : 1);
