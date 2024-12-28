use crate::{
    shader::{ShaderError, ShaderType},
    util,
};
use anyhow::Result;
use gl::types::GLuint;
use std::{marker::PhantomData, ops::Deref, ptr};

#[derive(Debug)]
pub struct ShaderCore<T> {
    shader: GLuint,
    _t: PhantomData<T>,
}

impl<T: ShaderType> ShaderCore<T> {
    pub fn new(filename: &str) -> Result<Self> {
        unsafe {
            let shader = gl::CreateShader(T::TYPE);
            let source = util::load_file(filename)?;
            gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let msg = util::get_log(shader, gl::GetShaderInfoLog)?;
                Err(ShaderError { msg })?
            }

            Ok(Self {
                shader,
                _t: Default::default(),
            })
        }
    }
}

impl<T> Drop for ShaderCore<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

impl<T> Deref for ShaderCore<T> {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}
