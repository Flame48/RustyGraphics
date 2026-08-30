use std::{ num::NonZeroU32, ops::{ Deref, DerefMut }, rc::Rc };

use softbuffer::{ Context, Surface };
use winit::window::Window;

use crate::{ application::buffer::ScreenBuffer2D, scene::renderer::FrameBuffer };

type Cell = u32;

pub struct WindowRenderingContext2D {
    buf: ScreenBuffer2D<Cell>,
    surface: Surface<Rc<Window>, Rc<Window>>,
}

impl Deref for WindowRenderingContext2D {
    type Target = ScreenBuffer2D<Cell>;
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for WindowRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
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
            buf: ScreenBuffer2D::new(size.width as usize, size.height as usize),
            surface,
        })
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let (Some(w), Some(h)) = (
            NonZeroU32::new(width as u32),
            NonZeroU32::new(height as u32),
        ) else {
            return;
        };
        self.buf.resize(width as usize, height as usize);
        let _ = self.surface.resize(w, h);
    }

    pub fn present(&mut self) {
        if let Ok(mut sb) = self.surface.buffer_mut() {
            sb.copy_from_slice(&self.buf.front); // Deref to front buffer, matching len
            self.buf.swap();
            let _ = sb.present();
        }
    }

    fn to_pixel(c: [u8; 4]) -> u32 {
        let [r, g, b, _a] = c;
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn blit_frame_buffer(&mut self, fb: &FrameBuffer) {
        if (fb.width as usize) == self.width && (fb.height as usize) == self.height {
            for (dst, &src) in self.back.iter_mut().zip(fb.color.iter()) {
                *dst = WindowRenderingContext2D::to_pixel(src);
            }
        }

        let cols = self.width().min(fb.width as usize);
        let rows = self.height().min(fb.height as usize);

        for y in 0..rows {
            for x in 0..cols {
                let idx = ((y as u32) * fb.width + (x as u32)) as usize;
                self.put(WindowRenderingContext2D::to_pixel(fb.color[idx]), x, y);
            }
        }
    }
}
