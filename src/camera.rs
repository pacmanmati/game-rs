#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

use cgmath::*;

pub struct Camera {
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
}

impl Camera {
    pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fovy,
            aspect,
            near,
            far,
            eye: (0.0, 3.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
        }
    }

    pub fn vp(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.near, self.far);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
