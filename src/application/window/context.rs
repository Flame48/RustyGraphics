use std::{ num::NonZeroU32, ops::{ Deref, DerefMut }, rc::Rc };

use softbuffer::{ Context, Surface };
use winit::window::Window;

use crate::{ application::buffer::ScreenBuffer2D, scene::renderer::FrameBuffer };

type Cell = u32;

pub struct WindowRenderingContext2D {
    surface: Surface<Rc<Window>, Rc<Window>>,
    width: usize,
    height: usize,
}

impl WindowRenderingContext2D {
    pub fn new(window: Rc<Window>) -> anyhow::Result<Self> {
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
            width: size.width.max(1) as usize,
            height: size.height.max(1) as usize,
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
        self.width = width;
        self.height = height;
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
            if
                (fb.width as usize) == (sb.width().get() as usize) &&
                (fb.height as usize) == (sb.height().get() as usize)
            {
                for (dst, &src) in sb.iter_mut().zip(fb.color.iter()) {
                    *dst = WindowRenderingContext2D::to_pixel(src);
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
