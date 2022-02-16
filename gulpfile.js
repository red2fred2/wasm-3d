const {src, dest} = require('gulp')
const clean = require('gulp-clean')
const {execSync} = require('child_process')
const glsl = require('gulp-glsl')
const rename = require('gulp-rename')

exports.default = cb => {
	const LIST = ["shaders", 'wasm', 'typescript', 'rollup', 'minify', 'html']
	for(task of LIST) execSync(`gulp ${task}`, {stdio: 'inherit'})
	cb()
}

const rm = list => {
	const OPTIONS = {
		read: false,
		allowEmpty: true
	}

	for(item of list) {
		console.log(`Cleaning ${item}`)
		src(item, OPTIONS)
		.pipe(clean())
	}
}

exports.nuke = cb => {
	this.clean(cb)
	rm([
		'node_modules',
		'Cargo.lock',
		'package-lock.json'
	])

	cb()
}

exports.clean = cb => {
	rm([
		'pkg',
		'target',
		'typescript/wasm',
		'typescript/*.js',
		'public',
		'web-build',
		'tsconfig.tsbuildinfo',
		'src/graphics/shaders/*.glsl-min'
	])

	cb()
}

exports.wasm = cb => {
	execSync('wasm-pack build --release --target web', {stdio: 'inherit'})
	execSync('wasm-strip pkg/*.wasm', {stdio: 'inherit'})
	src('pkg/*.ts').pipe(dest('typescript/wasm'))
	src('pkg/*.js').pipe(dest('typescript/wasm'))
	src('pkg/*.js').pipe(dest('web-build/wasm'))
	src('pkg/*.wasm').pipe(dest('public'))
	cb()
}

exports.typescript = cb => {
	execSync('tsc --incremental', {stdio: 'inherit'})
	src('typescript/*.js').pipe(dest('web-build'))
	cb()
}

exports.rollup = cb => {
	execSync('rollup web-build/script.js  --file web-build/bundle.js', {stdio: 'inherit'})
	cb()
}

exports.minify = cb => {
	execSync('terser --compress --mangle --toplevel -- web-build/bundle.js > public/script.js', {stdio: 'inherit'})
	// execSync('cp web-build/bundle.js public/script.js', {stdio: 'inherit'})
	cb()
}

exports.html = cb => {
	execSync('html-minifier-terser index.html --collapse-whitespace --minify-css > public/index.html', {stdio: 'inherit'})
	src('favicon.ico').pipe(dest('public'))
	cb()
}

exports.shaders = cb => {
	src('src/graphics/shaders/*.glsl')
		.pipe(glsl({format: 'raw'}))
		.pipe(rename(path => {path.extname = '.glsl-min'}))
		.pipe(dest('src/graphics/shaders'));
		cb()
}

exports.docs = cb => {
	execSync('cargo doc --document-private-items', {stdio: 'inherit'})
	cb()
}