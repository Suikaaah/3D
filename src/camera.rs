use glm::ext as gle;
use glm::Mat4;
use glm::Vec3;
use num_traits::Zero;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

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

    pub fn view(&self) -> Mat4 {
        gle::look_at(self.position, self.position + self.front(), self.up())
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

    pub fn mov_f(&mut self, delta: f32) {
        self.position = self.position + self.front_flat() * Self::SPEED * delta;
    }

    pub fn mov_b(&mut self, delta: f32) {
        self.position = self.position - self.front_flat() * Self::SPEED * delta;
    }

    pub fn mov_r(&mut self, delta: f32) {
        self.position = self.position + self.right_flat() * Self::SPEED * delta;
    }

    pub fn mov_l(&mut self, delta: f32) {
        self.position = self.position - self.right_flat() * Self::SPEED * delta;
    }

    pub fn mov_u(&mut self, delta: f32) {
        self.position = self.position + self.up_flat() * Self::SPEED * delta;
    }

    pub fn mov_d(&mut self, delta: f32) {
        self.position = self.position - self.up_flat() * Self::SPEED * delta;
    }

    pub fn input_kb(&mut self, state_kb: KeyboardState, delta: f32) {
        if state_kb.is_scancode_pressed(Scancode::W) {
            self.mov_f(delta);
        }
        if state_kb.is_scancode_pressed(Scancode::S) {
            self.mov_b(delta);
        }
        if state_kb.is_scancode_pressed(Scancode::A) {
            self.mov_l(delta);
        }
        if state_kb.is_scancode_pressed(Scancode::D) {
            self.mov_r(delta);
        }
        if state_kb.is_scancode_pressed(Scancode::Space) {
            self.mov_u(delta);
        }
        if state_kb.is_scancode_pressed(Scancode::LShift) {
            self.mov_d(delta);
        }
    }

    pub fn input_mouse(&mut self, dx: i32, dy: i32) {
        self.phi -= dx as f32 * Self::SENSITIVITY;
        self.theta = (self.theta + dy as f32 * Self::SENSITIVITY).clamp(-89.9, 89.9);
    }
}
