use crate::scene::math::matrix::{ Matrix, SqMat };

#[derive(Clone, Copy)]
pub struct Camera {
    width: u32,
    height: u32,
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            width,
            height,
            fov,
            z_near,
            z_far,
        }
    }

    pub fn projection_matrix(&self) -> SqMat<4> {
        let a = (self.height as f32) / (self.width as f32);
        let q = self.z_far / (self.z_far - self.z_near);
        let tfov = (self.fov * 0.5).tan();
        let f = 1.0 / tfov;

        SqMat::from_data([
            [a * f, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, q, 1.0],
            [0.0, 0.0, self.z_near * q, 0.0],
        ])
    }

    pub fn update_resolution(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
