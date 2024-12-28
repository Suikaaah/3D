use crate::util;
use anyhow::{anyhow, Result};
use sdl2::{render::Canvas, video::Window, EventPump, Sdl};
use std::{thread, time::Duration};

pub struct Handler {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Handler {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl_context = sdl2::init().map_err(|x| anyhow!(x))?;

        let video_subsystem = sdl_context.video().map_err(|x| anyhow!(x))?;

        let canvas = video_subsystem
            .window(title, width, height)
            .opengl()
            .build()
            .map_err(|x| anyhow!(x))?
            .into_canvas()
            .index(util::find_sdl_gl_driver().ok_or(anyhow!("find_sdl_gl_driver failed"))?)
            .build()
            .map_err(|x| anyhow!(x))?;

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        canvas
            .window()
            .gl_set_context_to_current()
            .expect("gl_set_context_to_current failed");

        Ok(Self {
            sdl_context,
            canvas,
        })
    }

    pub fn event_pump(&self) -> Result<EventPump> {
        self.sdl_context.event_pump().map_err(|x| anyhow!(x))
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn sleep(&self) {
        thread::sleep(Duration::new(0, 1_000_000_000 / 240));
    }
}
