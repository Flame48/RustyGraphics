use crate::scene::{
    math::matrix::{ RowMat, Transform },
    renderer::{ camera::Camera, fragment::Fragment },
};

pub struct Light {
    pub intensity: f32,
}

impl Light {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

pub trait LightingModel {
    fn shade(
        frag: &mut Fragment,
        view_transform: Transform,
        proj_transform: Transform,
        camera: &Camera,
        lights: &Vec<(&Light, Transform)>
    );
}

pub struct DiffuseLightingModel;
impl LightingModel for DiffuseLightingModel {
    fn shade(
        frag: &mut Fragment,
        view_projection: Transform,
        proj_transform: Transform,
        camera: &Camera,
        lights: &Vec<(&Light, Transform)>
    ) {
        const AMBIENT: f32 = 0.1;
        let normal = frag.normal.norm_row();
        let Some((light, light_transform)) = lights.get(0) else {
            return;
        };

        let origin = RowMat::<3>::new().to_homogenous();

        let light_pos_global = (
            origin * light_transform.extend_forward(view_projection).forward
        ).to_uniform();

        let light_pos = light_pos_global - frag.position;

        let light_distance = light_pos.mag_row();
        let light_dir = light_pos * (1.0 / light_distance);

        // let light_distance_attenuation = 1.0 / Self::attenuation_distance_function(light_distance);

        let light_direction_similarity = light_dir.dot(normal).max(0.0);
        let overall_light_intensity = (AMBIENT + light.intensity * light_direction_similarity).min(
            1.0
        );

        for i in 0..3 {
            frag.color[i] = ((frag.color[i] as f32) * overall_light_intensity).round() as u8;
        }
    }
}

pub struct PhongLightingModel;
impl LightingModel for PhongLightingModel {
    fn shade(
        frag: &mut Fragment,
        view_projection: Transform,
        proj_transform: Transform,
        camera: &Camera,
        lights: &Vec<(&Light, Transform)>
    ) {}
}
