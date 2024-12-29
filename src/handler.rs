use crate::util;
use anyhow::{anyhow, Result};
use sdl2::{render::Canvas, video::Window, EventPump, Sdl};
use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Handler {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    sleep_until: Instant,
    delta_counter: Instant,
}

impl Handler {
    const FPS: u32 = 275;
    const INTERVAL: Duration = Duration::new(0, 1_000_000_000 / Self::FPS);

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

        sdl_context.mouse().set_relative_mouse_mode(true);

        Ok(Self {
            sdl_context,
            canvas,
            sleep_until: Self::now(),
            delta_counter: Self::now(),
        })
    }

    pub fn event_pump(&self) -> Result<EventPump> {
        self.sdl_context.event_pump().map_err(|x| anyhow!(x))
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn sleep(&mut self) {
        thread::sleep(self.sleep_until - Self::now());
        self.sleep_until = Self::now() + Self::INTERVAL;
    }

    pub fn delta(&mut self) -> Duration {
        let retval = Self::now() - self.delta_counter;
        self.delta_counter = Self::now();
        retval
    }

    fn now() -> Instant {
        Instant::now()
    }
}
