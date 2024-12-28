use anyhow::Result;
use gl::types::{GLchar, GLsizei, GLuint};
use std::{ffi::CString, fs::File, io::Read};

pub fn find_sdl_gl_driver() -> Option<u32> {
    sdl2::render::drivers()
        .enumerate()
        .find(|(_, item)| item.name == "opengl")
        .map(|(index, _)| index as _)
}

pub fn get_log(
    target: u32,
    f: unsafe fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
) -> Result<String> {
    let mut buf = [0; 512];
    let mut length = 0;

    unsafe {
        f(target, buf.len() as _, &mut length, buf.as_mut_ptr());
    }

    Ok(
        CString::from_vec_with_nul(buf[0..=length as _].iter().map(|x| *x as _).collect())?
            .into_string()?,
    )
}

pub fn load_file(filename: &str) -> Result<CString> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    Ok(CString::new(buf)?)
}
