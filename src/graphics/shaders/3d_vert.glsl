// Attributes
attribute vec4 position;

// Uniforms
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Start shader
void main() {
	gl_Position = model * position;
}
