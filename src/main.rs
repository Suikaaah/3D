mod graphics;
mod handler;
mod shader;
mod util;

use glm::ext as gle;
use glm::{Mat4, Vec3};
use graphics::Graphics;
use graphics::LightCube;
use graphics::Lighting;
use handler::Handler;
use num_traits::identities::One;
use sdl2::event::Event;
use shader::shader_program::ShaderProgram;
use std::time::Instant;

fn main() {
    let (width, height) = (1280, 720);
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
    let camera = glm::vec3(0., 0., 3.);
    let fov = 90.;
    let front = glm::vec3(0., 0., -1.);
    let up = glm::vec3(0., 1., 0.);

    let mut event_pump = handler.event_pump().expect("failed to obtain event_pump");

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main_loop;
            }
        }

        unsafe {
            gl::ClearColor(0.1, 0.15, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mouse = event_pump.mouse_state();
        let light_pos = glm::vec3(
            (mouse.x() - width as i32 / 2) as f32 * 0.008,
            (mouse.y() - height as i32 / 2) as f32 * -0.008,
            1.,
        );
        let t = instant.elapsed().as_secs_f32();
        let projection =
            gle::perspective(glm::radians(fov), width as f32 / height as f32, 0.1, 100.);
        let view = gle::look_at(camera, camera + front, up);

        for i in 0..8 {
            let j = i as f32;
            let model = gle::scale(&Mat4::one(), Vec3::one() * 0.5);
            let model = gle::rotate(&model, j * 5. + t, glm::normalize(glm::vec3(1.3, 0.9, 1.5)));
            let model = gle::translate(&model, glm::vec3(t.sin(), j.sin(), (t + j).cos()));
            shader_lighting.enable();
            shader_lighting.set_vec3(c"objectColor", glm::vec3(0.31, 0.5, 1.));
            shader_lighting.set_vec3(c"lightColor", glm::vec3(1.0, 1.0, 1.0));
            shader_lighting.set_vec3(c"lightPos", light_pos);
            shader_lighting.set_vec3(c"viewPos", camera);
            shader_lighting.set_mat4(c"projection", &projection);
            shader_lighting.set_mat4(c"view", &view);
            shader_lighting.set_mat4(c"model", &model);
            lighting.draw();
        }

        let model = gle::scale(&gle::translate(&Mat4::one(), light_pos), Vec3::one() * 0.2);
        shader_light_cube.enable();
        shader_light_cube.set_mat4(c"projection", &projection);
        shader_light_cube.set_mat4(c"view", &view);
        shader_light_cube.set_mat4(c"model", &model);
        light_cube.draw();

        handler.present();
        handler.sleep();
    }
}
