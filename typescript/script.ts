import wasmInit, { InitOutput, WebApp } from "./wasm/wasm_engine.js"

let webApp: WebApp

// Kick off the whole thing as soon as the webassembly is ready
const init = async () => {
	const wasm: InitOutput = await wasmInit("./wasm_engine_bg.wasm")
	webApp = WebApp.init()

	// Set up render function
	const render = () => {
		webApp.render()
		window.requestAnimationFrame(render)
	}

	// Set up world update function
	let lastTime: number = Date.now()
	const update = () => {
		// Do timing
		const currentTime = Date.now()
		const dt: number = currentTime - lastTime
		lastTime = currentTime

		webApp.update(dt)
		window.setTimeout(update, 16.667)
	}

	update()
	render()
}

init()
