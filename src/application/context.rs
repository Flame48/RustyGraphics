use crate::application::cell::{ Cell, CellStyle };
use crate::scene::renderer::{ FrameBuffer, RGBA };

use std::io::{ self, Write };
use crossterm::{ cursor::MoveTo, queue, style::{ Print, SetBackgroundColor, SetForegroundColor } };

/// Console screen context. Exposes operations for manipulating the display, eg. fill, put, clear, etc.
pub struct Context {
    width: u16,
    height: u16,
    front: Vec<Cell>,
    back: Vec<Cell>,
}

impl Context {
    pub fn new(width: u16, height: u16) -> Self {
        let len = (width as usize) * (height as usize);
        Self {
            width,
            height,
            front: vec![Cell::default(); len],
            back: vec![Cell::default(); len],
        }
    }

    #[inline]
    pub fn idx(&self, x: u16, y: u16) -> Option<usize> {
        (x < self.width && y < self.height).then(
            || (y as usize) * (self.width as usize) + (x as usize)
        )
    }

    pub fn fill(&mut self, cell: Cell) {
        self.back.fill(cell);
    }

    pub fn clear(&mut self) {
        self.fill(Cell::default());
    }

    pub fn put(&mut self, cell: Cell, x: u16, y: u16) {
        if let Some(i) = self.idx(x, y) {
            self.back[i] = cell;
        }
    }

    pub fn fill_bound(&mut self, cell: Cell, x: u16, y: u16, w: u16, h: u16) {
        for row in y..y + h {
            for col in x..x + w {
                self.put(cell, col, row);
            }
        }
    }

    pub fn line(&mut self, cell: Cell, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (mut x0, mut y0) = (x0, y0);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.put(cell, x0 as u16, y0 as u16);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn triangle(&mut self, cell: Cell, p1: [i32; 2], p2: [i32; 2], p3: [i32; 2]) {
        self.line(cell, p1[0], p1[1], p2[0], p2[1]);
        self.line(cell, p2[0], p2[1], p3[0], p3[1]);
        self.line(cell, p3[0], p3[1], p1[0], p1[1]);
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn put_str(&mut self, s: &str, style: CellStyle, x: u16, y: u16) {
        for (i, ch) in s.chars().enumerate() {
            self.put(Cell { ch, style: style }, x + (i as u16), y);
        }
    }

    pub fn present(&mut self, out: &mut impl Write) -> io::Result<()> {
        let mut last_fg: Option<crossterm::style::Color> = None;
        let mut last_bg: Option<crossterm::style::Color> = None;
        let mut cursor: Option<(u16, u16)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y as usize) * (self.width as usize) + (x as usize);

                if self.back[i] == self.front[i] {
                    continue;
                }

                let cell = self.back[i];

                if cursor != Some((x, y)) {
                    queue!(out, MoveTo(x, y))?;
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
        std::mem::swap(&mut self.front, &mut self.back);
        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let len = (width as usize) * (height as usize);
        self.front = vec![Cell::default(); len];
        self.back = vec![Cell::default(); len];
    }

    fn to_color(c: RGBA) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: c[0], g: c[1], b: c[2] }
    }

    pub fn blit_frame_buffer(&mut self, fb: &FrameBuffer) {
        let cols = self.width().min(fb.width as u16);
        let rows = self.height().min((fb.height / 2) as u16);

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
                        style: CellStyle { fg: Context::to_color(top), bg: Context::to_color(bot) },
                    },
                    x,
                    y
                );
            }
        }
    }
}
