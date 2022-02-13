import { InitOutput } from "./wasm/wasm_engine"

let gamepads = {}

const checkGamepads = () => {
	navigator.getGamepads()
}

export function init(wasm:InitOutput) {
	// Handle keyboard press
	document.body.onkeydown = e => {

	}

	// Handle keyboard release
	document.body.onkeyup = e => {

	}

	// Gamepad connect
	window.addEventListener('gamepadconnected', e => {
		const gamepad: Gamepad = e.gamepad
		gamepads[gamepad.index] = gamepad

		console.log(`Gamepad ${gamepad.index} connected`)
		console.log(gamepad)
	})

	// Gamepad disconnect
	window.addEventListener('gamepaddisconnected', e => {
		const gamepad: Gamepad = e.gamepad
		delete gamepads[gamepad.index]

		console.log(`Gamepad ${gamepad.index} disconnected`)
	})

}
