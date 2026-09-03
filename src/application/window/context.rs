use std::{ num::NonZeroU32, rc::Rc };

use softbuffer::{ Context, Surface };
use winit::window::Window;

use crate::scene::renderer::FrameBuffer;

pub struct WindowRenderingContext2D {
    surface: Surface<Rc<Window>, Rc<Window>>,
    width: usize,
    height: usize,
}

impl WindowRenderingContext2D {
    pub fn new(window: Rc<Window>, render_w: usize, render_h: usize) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let context = Context::new(window.clone()).unwrap();

        let mut surface = Surface::new(&context, window).unwrap();
        surface
            .resize(
                NonZeroU32::new(size.width.max(1)).unwrap(),
                NonZeroU32::new(size.height.max(1)).unwrap()
            )
            .unwrap();

        Ok(Self {
            surface,
            width: render_w.max(1),
            height: render_h.max(1),
        })
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let (Some(w), Some(h)) = (
            NonZeroU32::new(width as u32),
            NonZeroU32::new(height as u32),
        ) else {
            return;
        };
        let _ = self.surface.resize(w, h);
        // self.width = width;
        // self.height = height;
    }

    fn to_pixel(c: [u8; 4]) -> u32 {
        let [r, g, b, _a] = c;
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn fill(&mut self, p: u32) {
        if let Ok(mut sb) = self.surface.buffer_mut() {
            sb.fill(p);
        }
    }

    pub fn clear(&mut self) {
        self.fill(0);
    }

    pub fn present(&mut self, fb: &FrameBuffer) {
        if let Ok(mut sb) = self.surface.buffer_mut() {
            let sb_width = sb.width().get() as usize;
            let sb_height = sb.height().get() as usize;
            let fb_width = fb.width as usize;
            let fb_height = fb.height as usize;
            if (fb_width as usize) == sb_width && fb_height == sb_height {
                for (dst, &src) in sb.iter_mut().zip(fb.color.iter()) {
                    *dst = WindowRenderingContext2D::to_pixel(src);
                }
            } else if fb_width > 0 && fb_height > 0 {
                for y in 0..sb_height {
                    let sy = (y * fb_height) / sb_height;
                    let src_row = sy * fb_width;
                    let dst_row = y * sb_width;
                    for x in 0..sb_width {
                        let sx = (x * fb_width) / sb_width;
                        sb[dst_row + x] = WindowRenderingContext2D::to_pixel(
                            fb.color[src_row + sx]
                        );
                    }
                }
            }
            let _ = sb.present();
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
