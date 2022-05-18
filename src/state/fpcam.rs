use super::window::WindowState;
use crate::event::Event;
use glam::{Mat4, Vec2, Vec3};
use winit::event::DeviceEvent;

//

/// First person camera controller
///
/// TODO: specify player (mouse | gamepad-id)
#[derive(Debug, Clone, Copy)]
pub struct FPCam {
    dir: Vec2,
    sensitivity: Vec2,
    fov: f32,
    near: f32,
    far: f32,
}

//

impl Default for FPCam {
    fn default() -> Self {
        Self {
            dir: Vec2::ZERO,
            sensitivity: Vec2::ONE * 0.003,
            fov: std::f32::consts::FRAC_PI_2,
            near: 0.1,
            far: 100.0,
        }
    }
}

impl FPCam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dir(mut self, dir: Vec2) -> Self {
        self.dir = Self::clamp2(dir);
        self
    }

    pub fn with_sensitivity(mut self, sensitivity: Vec2) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn with_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    pub fn with_near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }

    pub fn with_far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }

    pub fn get_dir(&self) -> Vec2 {
        Self::clamp2(self.dir)
    }

    pub fn get_matrix(&self, eye: Vec3, ws: &WindowState) -> Mat4 {
        let dir = self.get_dir();
        let center = Vec3::new(
            dir.y.cos() * dir.x.sin(),
            dir.y.sin(),
            dir.y.cos() * dir.x.cos(),
        );
        let up = Vec3::new(0.0, 1.0, 0.0);

        Mat4::perspective_rh(self.fov, ws.aspect, self.near, self.far)
            * Mat4::look_at_rh(eye, center, up)
    }

    pub fn update(&mut self, motion: Vec2) {
        self.dir += self.sensitivity * motion;
        self.clamp();
    }

    pub fn event(&mut self, event: &Event) {
        if let Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta: (x, y) },
            ..
        } = event
        {
            self.update(Vec2::new(*x as _, *y as _));
        }
    }

    //

    fn clamp(&mut self) {
        self.dir = Self::clamp2(self.dir);
    }

    fn clamp2(mut dir: Vec2) -> Vec2 {
        dir.y = dir.y.clamp(
            -std::f32::consts::PI / 2.0 + 0.01,
            std::f32::consts::PI / 2.0 - 0.01,
        );
        dir
    }
}
