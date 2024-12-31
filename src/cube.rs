use crate::{
    graphics::{Graphics, Lighting},
    shader::shader_program::ShaderProgram,
};
use glm::ext as gle;
use glm::{Mat4, Vec3};
use num_traits::{One, Zero};
use rand::distributions::Uniform;
use rand::Rng;

pub struct Cube {
    position: Vec3,
    velocity: Vec3,
    rotation: Vec3,
    rot_vel: Vec3,
    scale: f32,
}

impl Cube {
    pub fn new() -> Self {
        let uni_phi = Uniform::new(glm::radians(-180.), glm::radians(180.));
        let uni_r = Uniform::new(1.5, 4.);
        let uni_y = Uniform::new(-16., -12.);
        let uni_rot = Uniform::new(-4., 4.);

        let mut rng = rand::thread_rng();
        let mut f = |u| rng.sample(u);
        let phi: f32 = f(uni_phi);
        let r: f32 = f(uni_r);

        Self {
            position: Vec3::zero(),
            velocity: glm::vec3(r * phi.cos(), f(uni_y), r * phi.sin()),
            rotation: Vec3::zero(),
            rot_vel: glm::vec3(f(uni_rot), f(uni_rot), f(uni_rot)),
            scale: 0.5,
        }
    }

    pub fn floor() -> Self {
        Self {
            position: glm::vec3(0., 2.5, 0.),
            velocity: Vec3::zero(),
            rotation: Vec3::zero(),
            rot_vel: Vec3::zero(),
            scale: 4.,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.position.y > 25. {
            *self = Cube::new();
        }

        const GRAVITY: f32 = 10.;
        self.velocity = self.velocity + glm::vec3(0., GRAVITY, 0.) * delta;
        self.position = self.position + self.velocity * delta;
        self.rotation = self.rotation + self.rot_vel * delta;
    }

    pub fn draw(&self, lighting: &Graphics<Lighting>, shader_lighting: &ShaderProgram) {
        let model = gle::translate(&Mat4::one(), self.position);
        let model = gle::scale(&model, Vec3::one() * self.scale);
        let model = gle::rotate(&model, self.rotation.x, glm::vec3(1., 0., 0.));
        let model = gle::rotate(&model, self.rotation.y, glm::vec3(0., 1., 0.));
        let model = gle::rotate(&model, self.rotation.z, glm::vec3(0., 0., 1.));
        shader_lighting.set_mat4(c"model", &model);
        lighting.draw();
    }
}
