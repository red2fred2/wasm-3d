import wasmInit, { InitOutput } from "./wasm/wasm_engine.js"

// Kick off the whole thing as soon as the webassembly is ready
const init = async () => {
	const wasm:InitOutput = await wasmInit("./wasm_engine_bg.wasm")
	wasm.init_rs()
}

init()
