use crate::{
    shader::{shader_core::ShaderCore, Fragment, ShaderError, Vertex},
    util,
};
use anyhow::Result;
use gl::types::{GLint, GLuint};
use glm::{Mat4, Vec3};
use std::ffi::CStr;

#[derive(Debug)]
pub struct ShaderProgram {
    program: GLuint,
}

impl ShaderProgram {
    pub fn new(filename_v: &str, filename_f: &str) -> Result<Self> {
        unsafe {
            let shader_v = ShaderCore::<Vertex>::new(filename_v)?;
            let shader_f = ShaderCore::<Fragment>::new(filename_f)?;

            let program = gl::CreateProgram();

            gl::AttachShader(program, *shader_v);
            gl::AttachShader(program, *shader_f);
            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let msg = util::get_log(program, gl::GetProgramInfoLog)?;
                Err(ShaderError { msg })?
            }

            Ok(Self { program })
        }
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    fn locate(&self, name: &CStr) -> GLint {
        unsafe { gl::GetUniformLocation(self.program, name.as_ptr()) }
    }

    pub fn set_mat4(&self, name: &CStr, mat: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.locate(name),
                1,
                gl::FALSE,
                mat.as_array().as_ptr() as _,
            )
        }
    }

    pub fn set_vec3(&self, name: &CStr, vec: Vec3) {
        unsafe { gl::Uniform3fv(self.locate(name), 1, vec.as_array().as_ptr() as _) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}
