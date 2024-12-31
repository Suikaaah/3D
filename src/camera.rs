use glm::ext as gle;
use glm::{Mat4, Vec3};
use num_traits::Zero;
use sdl2::keyboard::{KeyboardState, Scancode};

pub struct Camera {
    pub position: Vec3,
    pub theta: f32,
    pub phi: f32,
}

impl Camera {
    const SPEED: f32 = 8.;
    const SENSITIVITY: f32 = 0.05;

    pub fn new() -> Self {
        Self {
            position: Vec3::zero(),
            theta: 0.,
            phi: 0.,
        }
    }

    pub fn view(&self) -> Mat4 {
        gle::look_at(self.position, self.position + self.front(), self.up())
    }

    pub fn input_kb(&mut self, state_kb: KeyboardState, delta: f32) {
        let mut process = |scancode, f: fn(&mut Self, f32)| {
            if state_kb.is_scancode_pressed(scancode) {
                f(self, delta);
            }
        };

        process(Scancode::W, Self::move_front);
        process(Scancode::S, Self::move_back);
        process(Scancode::D, Self::move_right);
        process(Scancode::A, Self::move_left);
        process(Scancode::Space, Self::move_up);
        process(Scancode::LShift, Self::move_down);
    }

    pub fn input_mouse(&mut self, dx: i32, dy: i32) {
        self.phi -= dx as f32 * Self::SENSITIVITY;
        self.theta = (self.theta + dy as f32 * Self::SENSITIVITY).clamp(-89.9, 89.9);
    }

    fn up_world() -> Vec3 {
        glm::vec3(0., -1., 0.)
    }

    fn front(&self) -> Vec3 {
        glm::normalize(glm::vec3(
            glm::radians(self.phi).cos() * glm::radians(self.theta).cos(),
            glm::radians(self.theta).sin(),
            glm::radians(self.phi).sin() * glm::radians(self.theta).cos(),
        ))
    }

    fn right(&self) -> Vec3 {
        glm::normalize(glm::cross(self.front(), Self::up_world()))
    }

    fn up(&self) -> Vec3 {
        glm::normalize(glm::cross(self.right(), self.front()))
    }

    fn front_flat(&self) -> Vec3 {
        glm::normalize(glm::vec3(
            glm::radians(self.phi).cos() * glm::radians(self.theta).cos(),
            0.,
            glm::radians(self.phi).sin() * glm::radians(self.theta).cos(),
        ))
    }

    fn right_flat(&self) -> Vec3 {
        glm::normalize(glm::cross(self.front_flat(), Self::up_world()))
    }

    fn up_flat(&self) -> Vec3 {
        glm::normalize(glm::cross(self.right_flat(), self.front_flat()))
    }

    fn move_front(&mut self, delta: f32) {
        self.position = self.position + self.front_flat() * Self::SPEED * delta;
    }

    fn move_back(&mut self, delta: f32) {
        self.position = self.position - self.front_flat() * Self::SPEED * delta;
    }

    fn move_right(&mut self, delta: f32) {
        self.position = self.position + self.right_flat() * Self::SPEED * delta;
    }

    fn move_left(&mut self, delta: f32) {
        self.position = self.position - self.right_flat() * Self::SPEED * delta;
    }

    fn move_up(&mut self, delta: f32) {
        self.position = self.position + self.up_flat() * Self::SPEED * delta;
    }

    fn move_down(&mut self, delta: f32) {
        self.position = self.position - self.up_flat() * Self::SPEED * delta;
    }
}
