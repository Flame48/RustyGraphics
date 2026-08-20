use crate::scene::math::matrix::{ SqMat, Transform };

#[derive(Clone, Copy)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
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

    pub fn projection_transform(&self) -> Transform {
        let a = (self.height as f32) / (self.width as f32);
        let q = self.z_far / (self.z_far - self.z_near);
        let tfov = (self.fov * 0.5).tan();
        let f = 1.0 / tfov;
        let af = a * f;
        let zq = self.z_near * q;

        let f_inv = 1.0 / f;
        let af_inv = 1.0 / af;
        let z_inv = 1.0 / self.z_near;
        let zq_inv = (1.0 / self.z_near) * q;

        Transform {
            forward: SqMat::<4>::from_data([
                [af, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, q, 1.0],
                [0.0, 0.0, -zq, 0.0],
            ]),
            reverse: SqMat::<4>::from_data([
                [af_inv, 0.0, 0.0, 0.0],
                [0.0, f_inv, 0.0, 0.0],
                [0.0, 0.0, 0.0, -zq_inv],
                [0.0, 0.0, 1.0, z_inv],
            ]),
        }
    }

    pub fn update_resolution(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
