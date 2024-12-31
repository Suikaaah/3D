mod camera;
mod cube;
mod graphics;
mod handler;
mod shader;
mod util;

use camera::Camera;
use cube::Cube;
use glm::ext as gle;
use glm::{Mat4, Vec3};
use graphics::{Graphics, LightCube, Lighting};
use handler::Handler;
use num_traits::identities::One;
use rand::distributions::Uniform;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use shader::shader_program::ShaderProgram;
use std::array;
use std::time::Instant;

fn main() {
    let (width, height) = (1600, 900);
    let aspect_ratio = width as f32 / height as f32;
    let mut handler = Handler::new("Window", width, height).expect("could not obtain handler");

    let lighting = Graphics::<Lighting>::new(graphics::CUBE);
    let light_cube = Graphics::<LightCube>::new(graphics::CUBE);

    let shader_lighting =
        ShaderProgram::new("lighting.vs", "lighting.fs").expect("could not obtain shader_lighting");
    let shader_light_cube =
        ShaderProgram::new("light_cube.vs", "light_cube.fs").expect("could not obtain light_cube");

    let mut camera = Camera::new();
    let mut cubes: [Cube; 256] = array::from_fn(|_| Cube::new());
    const LIGHTS: usize = 225;
    let light_positions: [Vec3; LIGHTS] = array::from_fn(|i| {
        let i = i as i32;
        let x = i / 15 - 7;
        let z = i % 15 - 7;
        glm::vec3((x * 12) as f32, 0., (z * 12) as f32)
    });
    let light_colors: [Vec3; LIGHTS] = {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0., glm::radians(360.));

        array::from_fn(|_| {
            let theta = rng.sample(uniform);
            let c = |offset: f32| 0.75 + 0.25 * (theta + glm::radians(offset)).sin();
            glm::vec3(c(0.), c(120.), c(240.))
        })
    };

    let floor = Cube::floor();
    let object_color = glm::vec3(1., 1., 1.);
    let fog_color = glm::vec3(0.2, 0.25, 0.3);
    let ambient_color = fog_color * 0.2;
    let projection = gle::perspective(glm::radians(90.), aspect_ratio, 0.1, 100.);

    let instant = Instant::now();
    let mut event_pump = handler.event_pump().expect("failed to obtain event_pump");

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

        let dt = handler.delta().as_secs_f32();
        let t = instant.elapsed().as_secs_f32();
        camera.input_kb(event_pump.keyboard_state(), dt);
        let force_mult = {
            let ms = event_pump.mouse_state();
            if ms.is_mouse_button_pressed(MouseButton::Left) {
                1.
            } else if ms.is_mouse_button_pressed(MouseButton::Right) {
                -1.
            } else {
                0.
            }
        } * 1000.;

        unsafe {
            gl::ClearColor(fog_color.x, fog_color.y, fog_color.z, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_lighting.enable();
        shader_lighting.set_f32(c"time", t);
        shader_lighting.set_u32(c"lights", LIGHTS as _);
        shader_lighting.set_vec3(c"objectColor", object_color);
        shader_lighting.set_vec3(c"fogColor", fog_color);
        shader_lighting.set_vec3(c"ambientColor", ambient_color);
        shader_lighting.set_vec3(c"viewPos", camera.position);
        shader_lighting.set_vec3_array(c"lightPositions", &light_positions);
        shader_lighting.set_vec3_array(c"lightColors", &light_colors);
        shader_lighting.set_mat4(c"projection", &projection);
        shader_lighting.set_mat4(c"view", &camera.view());
        floor.draw(&lighting, &shader_lighting);

        for cube in &mut cubes {
            let diff = camera.position + camera.front() * 16. - cube.position;
            cube.update(dt, (glm::normalize(diff) / glm::length(diff)) * force_mult);
            cube.draw(&lighting, &shader_lighting);
        }

        shader_light_cube.enable();
        shader_light_cube.set_mat4(c"view", &camera.view());
        shader_light_cube.set_mat4(c"projection", &projection);

        for (pos, color) in light_positions.into_iter().zip(light_colors) {
            let model = gle::scale(&gle::translate(&Mat4::one(), pos), Vec3::one() * 0.1);
            shader_light_cube.set_vec3(c"lightColor", color);
            shader_light_cube.set_mat4(c"model", &model);
            light_cube.draw();
        }

        handler.present();
        handler.sleep();
    }
}
