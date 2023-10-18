const fs = require("fs")

interface WasmInfo {
  imports: {
    from: string;
    names: string[];
  }[];
  exports: string[];
}

async function parseWasm(wasmFilePath: string): Promise<WasmInfo> {
  try {
    const wasmBinary = await fs.promises.readFile(wasmFilePath);
    const wasmModule = await WebAssembly.compile(wasmBinary);
    const imports = Object.entries(
      WebAssembly.Module.imports(wasmModule).reduce(
        (result, item) => ({
          ...result,
          [item.module]: [...(result[item.module] || []), item.name]
        }),
        {} as Record<string, string[]>
      )
    ).map(([from, names]) => ({ from, names }));

    const exports = WebAssembly.Module.exports(wasmModule).map(item => item.name);

    return { imports, exports };
  } catch (e) {
    throw new Error(`Failed to parse WASM file: ${e.message}`);
  }
}

async function generateWrapperCode(
  wasmFilePath: string,
): Promise<string> {
  const { imports } = await parseWasm(wasmFilePath);
  return `
import { __wbg_set_wasm } from '.'
import wasm from "./kdtree_wasm_bg.wasm"
${imports
  .map(
    ({ from, names }) =>
      `import { ${names.map((name) => `${name}`).join(", ")} } from ${JSON.stringify(
        from
      )};`
  )
  .join("\n")}
const wasmModule = { ${imports
    .map(
      ({ from, names }) =>
        `${JSON.stringify(from)}: { ${names.map((name) => `${name}: ${name}`).join(", ")} }`
    )
    .join(", ")} };

export async function initWasm() {
  const a = await WebAssembly.instantiateStreaming(fetch(wasm), wasmModule)
  __wbg_set_wasm(a.instance.exports)
}
`;
}

function generateWrapperTypeCode() {
  return `/* tslint:disable */
/* eslint-disable */
export function initWasm(): Promise<void>
`
}


// excute
(async () => {
  const jsContent = await generateWrapperCode("./dist/bundler/kdtree_wasm_bg.wasm")
  await fs.promises.writeFile("./dist/bundler/kdtree_wasm_wrapper.js", jsContent)
  await fs.promises.writeFile("./dist/bundler/kdtree_wasm_wrapper.d.ts", generateWrapperTypeCode())
})();


