use crate::application::buffer::ScreenBuffer2D;
use crate::application::console::cell::{ Cell, CellStyle };
use crate::scene::renderer::{ FrameBuffer, RGBA };

use std::io::{ self, Write };
use std::ops::{ Deref, DerefMut };
use crossterm::{ cursor::MoveTo, queue, style::{ Print, SetBackgroundColor, SetForegroundColor } };

/// Console screen context. Exposes operations for manipulating the display, eg. fill, put, clear, etc.
pub struct ConsoleRenderingContext2D {
    buf: ScreenBuffer2D<Cell>,
}

impl Deref for ConsoleRenderingContext2D {
    type Target = ScreenBuffer2D<Cell>;
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for ConsoleRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

impl ConsoleRenderingContext2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self { buf: ScreenBuffer2D::new(width, height) }
    }

    pub fn put_str(&mut self, s: &str, style: CellStyle, x: u16, y: u16) {
        for (i, ch) in s.chars().enumerate() {
            self.put(Cell { ch, style: style }, (x as usize) + i, y as usize);
        }
    }

    pub fn present(&mut self, out: &mut impl Write) -> io::Result<()> {
        let mut last_fg: Option<crossterm::style::Color> = None;
        let mut last_bg: Option<crossterm::style::Color> = None;
        let mut cursor: Option<(usize, usize)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y as usize) * (self.width as usize) + (x as usize);

                let cell = self.buffer[i];

                if cursor != Some((x, y)) {
                    queue!(out, MoveTo(x as u16, y as u16))?;
                }

                if last_fg != Some(cell.style.fg) {
                    queue!(out, SetForegroundColor(cell.style.fg))?;
                    last_fg = Some(cell.style.fg);
                }

                if last_bg != Some(cell.style.bg) {
                    queue!(out, SetBackgroundColor(cell.style.bg))?;
                    last_bg = Some(cell.style.bg);
                }

                queue!(out, Print(cell.ch))?;

                cursor = Some((x + 1, y));
            }
        }

        out.flush()?;

        Ok(())
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let len = (width as usize) * (height as usize);
        self.buffer = vec![Cell::default(); len];
    }

    fn to_color(c: RGBA) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: c[0], g: c[1], b: c[2] }
    }

    pub fn blit_frame_buffer(&mut self, fb: &FrameBuffer) {
        let cols = self.width().min(fb.width as usize);
        let rows = self.height().min((fb.height / 2) as usize);

        for y in 0..rows {
            for x in 0..cols {
                let top_idx = ((y as u32) * 2 * fb.width + (x as u32)) as usize;
                let bot_idx = (((y as u32) * 2 + 1) * fb.width + (x as u32)) as usize;

                let top = fb.color[top_idx];
                let bot = fb.color[bot_idx];

                let ch = if top == bot { '█' } else { '▀' };

                self.put(
                    Cell {
                        ch,
                        style: CellStyle {
                            fg: ConsoleRenderingContext2D::to_color(top),
                            bg: ConsoleRenderingContext2D::to_color(bot),
                        },
                    },
                    x,
                    y
                );
            }
        }
    }
}
