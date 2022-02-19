// Attributes
attribute vec4 position;

// Uniforms
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Start shader
void main() {
	mat4 mvp_matrix = projection * view * model;

	gl_Position = mvp_matrix * position;
}
