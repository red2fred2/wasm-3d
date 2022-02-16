import wasmInit, { InitOutput, WebApp } from "./wasm/wasm_engine.js"

const DEBUG = true

let webApp: WebApp

// Kick off the whole thing as soon as the webassembly is ready
const init = async () => {
	const wasm: InitOutput = await wasmInit("./wasm_engine_bg.wasm")
	webApp = WebApp.init()

	// Add fps monitoring
	let lastTimeRender: number = Date.now()
	let renderDt = []
	const displayFps = () => {
		if(DEBUG) {
			const rollingAvg = renderDt.reduce((a, b) => a + b) / renderDt.length
			console.log(`FPS: ${Math.round(1000/rollingAvg)}`)
		}
	}

	// Set up render function
	const render = () => {
		if(DEBUG) {
			// Calculate FPS
			const currentTime: number = Date.now()
			const dt: number = currentTime - lastTimeRender
			lastTimeRender = currentTime
			if(renderDt.length > 100) renderDt.shift()
			renderDt.push(dt)
		}

		// Render
		webApp.render()
		window.requestAnimationFrame(render)
	}

	// Set up world update function
	let lastTimeUpdate: number = Date.now()
	const update = () => {
		// Do timing
		const currentTime: number = Date.now()
		const dt: number = currentTime - lastTimeUpdate
		lastTimeUpdate = currentTime

		webApp.update(dt)
		window.setTimeout(update, 16.667)
	}

	update()
	render()
	setInterval(displayFps, 1000)
}

init()
