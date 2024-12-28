mod shader_core;
pub mod shader_program;

use gl::types::GLenum;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct ShaderError {
    msg: String,
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ShaderError {}

#[derive(Debug)]
struct Vertex;

#[derive(Debug)]
struct Fragment;

trait ShaderType {
    const TYPE: GLenum;
}

impl ShaderType for Vertex {
    const TYPE: GLenum = gl::VERTEX_SHADER;
}

impl ShaderType for Fragment {
    const TYPE: GLenum = gl::FRAGMENT_SHADER;
}
