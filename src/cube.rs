use crate::{
    graphics::{Graphics, Lighting},
    shader::shader_program::ShaderProgram,
    util,
};
use glm::ext as gle;
use glm::{Mat4, Vec3};
use num_traits::{One, Zero};
use rand::distributions::Uniform;
use rand::Rng;

pub struct Cube {
    pub position: Vec3,
    velocity: Vec3,
    rot_axis: Vec3,
    rotation: f32,
    rot_speed: f32,
    scale: f32,
}

impl Cube {
    pub fn new() -> Self {
        let uni_r = Uniform::new(12., 24.);
        let uni_phi = Uniform::new(glm::radians(-180.), glm::radians(180.));
        let uni_theta = Uniform::new(glm::radians(-90.), glm::radians(-60.));
        let uni_rot_phi = Uniform::new(glm::radians(-180.), glm::radians(180.));
        let uni_rot_theta = Uniform::new(glm::radians(-90.), glm::radians(90.));
        let uni_rot_speed = Uniform::new(0., 8.);

        let mut rng = rand::thread_rng();
        let mut f = |u| rng.sample(u);
        let r: f32 = f(uni_r);
        let phi: f32 = f(uni_phi);
        let theta: f32 = f(uni_theta);
        let rot_phi: f32 = f(uni_rot_phi);
        let rot_theta: f32 = f(uni_rot_theta);
        let rot_speed: f32 = f(uni_rot_speed);

        Self {
            position: glm::vec3(0., -1., 0.),
            velocity: util::sphere(theta, phi) * r,
            rot_axis: util::sphere(rot_theta, rot_phi),
            rotation: 0.,
            rot_speed,
            scale: 0.75,
        }
    }

    pub fn floor() -> Self {
        Self {
            position: glm::vec3(0., 500.5, 0.),
            velocity: Vec3::zero(),
            rot_axis: Vec3::one(),
            rotation: 0.,
            rot_speed: 0.,
            scale: 1000.,
        }
    }

    pub fn update(&mut self, dt: f32, force: Vec3) {
        if self.position.y > 5. {
            *self = Cube::new();
        }

        const GRAVITY: f32 = 10.;
        self.velocity = self.velocity + (force + glm::vec3(0., GRAVITY, 0.)) * dt;
        self.position = self.position + self.velocity * dt;
        self.rotation += self.rot_speed * dt;
    }

    pub fn draw(&self, lighting: &Graphics<Lighting>, shader_lighting: &ShaderProgram) {
        let model = gle::translate(&Mat4::one(), self.position);
        let model = gle::scale(&model, Vec3::one() * self.scale);
        let model = gle::rotate(&model, self.rotation, self.rot_axis);
        shader_lighting.set_mat4(c"model", &model);
        lighting.draw();
    }
}
