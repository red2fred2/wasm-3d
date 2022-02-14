use std::collections::HashMap;

use super::ShaderSource;

pub fn get_shader_sources() -> HashMap<&'static str, ShaderSource<'static>> {
	let mut sources = HashMap::new();

	// Basic bitch shader
	sources.insert("Basic bitch", ShaderSource {
		vertex_shader: Some(include_str!("here_vert.glsl-min")),
		fragment_shader: Some(include_str!("orange_frag.glsl-min"))
	});

	sources
}
