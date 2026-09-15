use std::path::Path;
use image::{ GenericImageView, ImageReader };

use crate::scene::{ renderer::RGBA, sampler::SamplerTarget };

pub struct ImageTexture {
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageTexture {
    pub fn import(path: impl AsRef<Path>) -> Result<Self, String> {
        let im = ImageReader::open(path)
            .map_err(|e| format!("Unable to open texture file: {}", e.to_string()))?
            .decode()
            .map_err(|_| format!("Unable to decode texture file"))?;
        let (width_u32, height_u32) = im.dimensions();
        let raw = im.to_rgba8().into_raw();

        Ok(Self { data: raw, width: width_u32 as usize, height: height_u32 as usize })
    }
}

impl SamplerTarget for ImageTexture {
    fn sample(&self, u: f32, v: f32) -> RGBA {
        let xi: usize = ((u * (self.width as f32)).round() as usize).clamp(0, self.width - 1);
        let yi: usize = (((1.0 - v) * (self.height as f32)).round() as usize).clamp(
            0,
            self.height - 1
        );

        let i = (yi * self.width + xi) * 4;

        self.data[i..i + 4].try_into().expect("Unable to sample texture!")
    }
}
