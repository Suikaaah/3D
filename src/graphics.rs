use gl::types::{GLsizei, GLuint};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Graphics<T> {
    vbo: GLuint,
    vao: GLuint,
    vertex_count: usize,
    _t: PhantomData<T>,
}

#[derive(Debug)]
pub struct Lighting;
#[derive(Debug)]
pub struct LightCube;

impl<T> Graphics<T> {
    const F32_SIZE: GLsizei = size_of::<f32>() as _;

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as _);
        }
    }
}

impl Graphics<Lighting> {
    pub fn new(vertices: &[f32]) -> Self {
        let (mut vbo, mut vao) = (0, 0);

        unsafe {
            gl::GenBuffers(1, &mut vbo as _);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(vertices) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut vao as _);
            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * Self::F32_SIZE, 0 as _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * Self::F32_SIZE,
                (3 * Self::F32_SIZE) as _,
            );
            gl::EnableVertexAttribArray(1);
        }

        Self {
            vbo,
            vao,
            vertex_count: vertices.len() / 6,
            _t: Default::default(),
        }
    }
}

impl Graphics<LightCube> {
    pub fn new(vertices: &[f32]) -> Self {
        let (mut vbo, mut vao) = (0, 0);

        unsafe {
            gl::GenBuffers(1, &mut vbo as _);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(vertices) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut vao as _);
            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * Self::F32_SIZE, 0 as _);
            gl::EnableVertexAttribArray(0);
        }

        Self {
            vbo,
            vao,
            vertex_count: vertices.len() / 6,
            _t: Default::default(),
        }
    }
}

impl<T> Drop for Graphics<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao as _);
            gl::DeleteBuffers(1, &mut self.vbo as _);
        }
    }
}
