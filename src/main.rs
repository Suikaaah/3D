mod camera;
mod graphics;
mod handler;
mod shader;
mod util;

use camera::Camera;
use glm::ext as gle;
use glm::{Mat4, Vec3};
use graphics::Graphics;
use graphics::LightCube;
use graphics::Lighting;
use handler::Handler;
use num_traits::identities::One;
use num_traits::Zero;
use rand::distributions::Uniform;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use shader::shader_program::ShaderProgram;
use std::array;
use std::time::Instant;

struct Cube {
    position: Vec3,
    velocity: Vec3,
    angle: Vec3,
    ang_vel: Vec3,
}

impl Cube {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let xz = Uniform::new(8., 12.);
        let y = Uniform::new(-8., -4.);
        let velocity = glm::vec3(rng.sample(xz), rng.sample(y), rng.sample(xz));

        let a = Uniform::new(-2., 2.);
        let ang_vel = glm::vec3(rng.sample(a), rng.sample(a), rng.sample(a));

        Self {
            position: Vec3::zero(),
            velocity,
            angle: Vec3::zero(),
            ang_vel,
        }
    }

    fn update(&mut self, delta: f32) {
        const GRAVITY: f32 = 0.02;
        self.velocity = self.velocity + glm::vec3(0., GRAVITY, 0.);
        self.position = self.position + self.velocity * delta;
        self.angle = self.angle + self.ang_vel * delta;
    }
}

fn main() {
    let (width, height) = (1600, 900);
    let aspect_ratio = width as f32 / height as f32;
    let mut handler = Handler::new("Window", width, height).expect("could not obtain handler");

    let cube: &[f32] = &[
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, 0.5, -0.5, 0.0,
        0.0, -1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, -0.5,
        -0.5, 0.0, 0.0, -1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5,
        0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5,
        -0.5, 0.5, 0.0, 0.0, 1.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, 0.5, -1.0,
        0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5, -0.5,
        1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5,
        0.5, 1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5,
        -0.5, -0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, -0.5, 0.5, -0.5, 0.0,
        1.0, 0.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0,
        1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
    ];

    let lighting = Graphics::<Lighting>::new(cube);
    let light_cube = Graphics::<LightCube>::new(cube);

    let shader_lighting =
        ShaderProgram::new("lighting.vs", "lighting.fs").expect("could not obtain shader_lighting");
    let shader_light_cube =
        ShaderProgram::new("light_cube.vs", "light_cube.fs").expect("could not obtain light_cube");

    let instant = Instant::now();
    let mut camera = Camera::new();
    let mut event_pump = handler.event_pump().expect("failed to obtain event_pump");
    let light_pos = glm::vec3(0., 0., 0.);

    let mut cubes: [Cube; 64] = array::from_fn(|_| Cube::new());

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::MouseMotion { xrel, yrel, .. } => {
                    camera.input_mouse(xrel, yrel);
                }
                _ => {}
            }
        }

        let delta = handler.delta().as_secs_f32();
        camera.input_kb(event_pump.keyboard_state(), delta);

        unsafe {
            gl::ClearColor(0.1, 0.15, 0.2, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let t = instant.elapsed().as_secs_f32();
        let projection = gle::perspective(glm::radians(90.), aspect_ratio, 0.1, 100.);
        let model = gle::scale(
            &gle::translate(&Mat4::one(), glm::vec3(0., 25.5, 0.)),
            Vec3::one() * 50.,
        );
        shader_lighting.enable();
        shader_lighting.set_f32(c"time", t);
        shader_lighting.set_vec3(c"objectColor", glm::vec3(1., 1., 1.));
        shader_lighting.set_vec3(c"lightColor", glm::vec3(1., 0.95, 0.9));
        shader_lighting.set_vec3(c"lightPos", light_pos);
        shader_lighting.set_vec3(c"viewPos", camera.position);
        shader_lighting.set_mat4(c"projection", &projection);
        shader_lighting.set_mat4(c"view", &camera.view());
        shader_lighting.set_mat4(c"model", &model);
        lighting.draw();

        for cube in &mut cubes {
            if cube.position.y > 1. {
                *cube = Cube::new();
            }
            cube.update(delta);
            let model = gle::translate(&Mat4::one(), cube.position);
            let model = gle::scale(&model, Vec3::one() * 0.4);
            let model = gle::rotate(&model, 1., cube.angle);
            shader_lighting.set_mat4(c"model", &model);
            lighting.draw();
        }

        let model = gle::scale(&gle::translate(&Mat4::one(), light_pos), Vec3::one() * 0.1);
        shader_light_cube.enable();
        shader_light_cube.set_mat4(c"projection", &projection);
        shader_light_cube.set_mat4(c"view", &camera.view());
        shader_light_cube.set_mat4(c"model", &model);
        light_cube.draw();

        handler.present();
        handler.sleep();
    }
}
