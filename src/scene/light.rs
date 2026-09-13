use std::ops::{ Add, AddAssign, Mul, Sub };

use crate::scene::{
    math::matrix::{ RowMat, Transform },
    renderer::fragment::Fragment,
    camera::Camera,
};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    const WHITE: Color = Color::new(255.0, 255.0, 255.0, 255.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xff) as u8;
        let g = ((hex >> 16) & 0xff) as u8;
        let b = ((hex >> 8) & 0xff) as u8;
        let a = (hex & 0xff) as u8;

        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn from_u8(c: [u8; 4]) -> Self {
        Self::new(c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32)
    }

    pub const fn from_rgb_u8(c: [u8; 3]) -> Self {
        Self::rgb(c[0] as f32, c[1] as f32, c[2] as f32)
    }

    pub fn to_rgb_u8(self) -> [u8; 3] {
        let c = self.clamped();
        [c.r as u8, c.g as u8, c.b as u8]
    }

    pub fn to_rgba_u8(self) -> [u8; 4] {
        let c = self.clamped();
        [c.r as u8, c.g as u8, c.b as u8, c.a as u8]
    }

    pub fn apply(self, f: impl Fn(f32) -> f32) -> Self {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }

    pub fn apply_zip(self, other: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Self {
            r: f(self.r, other.r),
            g: f(self.g, other.g),
            b: f(self.b, other.b),
            a: f(self.a, other.a),
        }
    }

    pub fn apply_rgb(self, f: impl Fn(f32) -> f32) -> Self {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: self.a }
    }

    pub fn apply_rgb_zip(self, other: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Self {
            r: f(self.r, other.r),
            g: f(self.g, other.g),
            b: f(self.b, other.b),
            a: self.a,
        }
    }

    pub fn apply_mut(&mut self, f: impl Fn(f32) -> f32) {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = f(self.a);
    }

    pub fn apply_zip_mut(&mut self, other: Self, f: impl Fn(f32, f32) -> f32) {
        self.r = f(self.r, other.r);
        self.g = f(self.g, other.g);
        self.b = f(self.b, other.b);
        self.a = f(self.a, other.a);
    }

    pub fn apply_rgb_mut(&mut self, f: impl Fn(f32) -> f32) {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
    }

    pub fn apply_rgb_zip_mut(&mut self, other: Self, f: impl Fn(f32, f32) -> f32) {
        self.r = f(self.r, other.r);
        self.g = f(self.g, other.g);
        self.b = f(self.b, other.b);
    }

    pub fn clamped(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 255.0),
            g: self.g.clamp(0.0, 255.0),
            b: self.b.clamp(0.0, 255.0),
            a: self.a.clamp(0.0, 255.0),
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        self.apply_rgb_zip(rhs, |x, y| x + y)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.apply_rgb_zip_mut(rhs, |x, y| x + y)
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        self.apply_rgb_zip(rhs, |x, y| x - y)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        self.apply_rgb(|x| x * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        self.apply_rgb_zip(rhs, |x, y| x * y)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

pub struct Light {
    pub intensity: f32,
    pub color: Color,
}

impl Light {
    pub fn new(intensity: f32) -> Self {
        Self { intensity, color: Color::WHITE }
    }

    pub fn new_color(intensity: f32, color: Color) -> Self {
        Self { intensity, color: color }
    }
}

pub trait LightingModel {
    fn shade(
        frag: &mut Fragment,
        view_transform: Transform,
        proj_transform: Transform,
        camera: &Camera,
        camera_pos_global: RowMat<3>,
        lights: &Vec<(&Light, RowMat<3>)>
    );
}

pub struct DiffuseLightingModel;

impl DiffuseLightingModel {
    const ATTENUATION_C1: f32 = 0.5;
    const ATTENUATION_C2: f32 = 0.5;
}

impl LightingModel for DiffuseLightingModel {
    fn shade(
        frag: &mut Fragment,
        _view_transform: Transform,
        _proj_transform: Transform,
        _camera: &Camera,
        camera_pos_global: RowMat<3>,
        lights: &Vec<(&Light, RowMat<3>)>
    ) {
        const AMBIENT: f32 = 0.1;
        const SHININESS: f32 = 10.0;

        let normal = frag.normal.norm_row();

        let mat_color = Color::from_u8(frag.color);

        let mut color = mat_color * AMBIENT;

        let camera_pos = camera_pos_global - frag.position;
        let camera_distance = camera_pos.mag_row();
        let camera_dir = camera_pos * (1.0 / camera_distance);

        for (light, light_pos_global) in lights {
            let light_pos = *light_pos_global - frag.position;

            let light_distance = light_pos.mag_row();
            let light_dir = light_pos * (1.0 / light_distance);

            let inv_light_distance_attenuation =
                1.0 +
                Self::ATTENUATION_C1 * light_distance +
                Self::ATTENUATION_C2 * light_distance.powi(2);

            let light_distance_attenuation = 1.0 / inv_light_distance_attenuation;

            let light_direction_similarity = light_dir.dot(normal).max(0.0);

            let diffuse_color = mat_color * light.color * light_direction_similarity;

            let reflection = normal * 2.0 * light_direction_similarity - light_dir;
            let reflection_direction_similarity = reflection
                .dot(camera_dir)
                .clamp(0.0, 1.0)
                .powf(SHININESS);

            let specular_color = light.color * reflection_direction_similarity;
            color += (
                light_distance_attenuation *
                light.intensity *
                (diffuse_color + specular_color)
            ).clamped();
        }

        frag.color = color.to_rgba_u8();
    }
}
