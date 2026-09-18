use std::path::Path;
use image::{ GenericImageView, ImageReader };

use crate::scene::{ renderer::RGBA, sampler::SamplerTarget };

pub struct ImageTexture {
    data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    pub fn import(path: impl AsRef<Path>) -> Result<Self, String> {
        let im = ImageReader::open(path)
            .map_err(|e| format!("Unable to open texture file: {}", e.to_string()))?
            .decode()
            .map_err(|_| format!("Unable to decode texture file"))?;
        let (width, height) = im.dimensions();
        let raw = im.to_rgba8().into_raw();

        // TODO: Store texels in a more cache efficient format
        // See (https://fgiesen.wordpress.com/2011/01/17/texture-tiling-and-swizzling/)

        Ok(Self { data: raw, width, height })
    }
}

impl SamplerTarget for ImageTexture {
    fn sample(&self, u: f32, v: f32) -> RGBA {
        let xi: u32 = ((u * (self.width as f32)).round() as u32).clamp(0, self.width - 1);
        let yi: u32 = (((1.0 - v) * (self.height as f32)).round() as u32).clamp(0, self.height - 1);

        let i = ((yi * self.width + xi) * 4) as usize;

        self.data[i..i + 4].try_into().expect("Unable to sample texture!")
    }
}
