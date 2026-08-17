use std::io::{ self, Write };
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ Color, Print, SetBackgroundColor, SetForegroundColor },
};

#[derive(Clone, Copy, PartialEq, Eq)]
struct CellStyle {
    fg: Color,
    bg: Color,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    ch: char,
    style: CellStyle,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self { fg: Color::White, bg: Color::Black }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', style: CellStyle::default() }
    }
}

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

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn put_str(&mut self, s: &str, style: CellStyle, x: u16, y: u16) {
        for (i, ch) in s.chars().enumerate() {
            self.put(Cell { ch, style: CellStyle::default() }, x + (i as u16), y);
        }
    }

    fn present(&mut self, out: &mut impl Write) -> io::Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y as usize) * (self.width as usize) + (x as usize);
                if self.back[i] != self.front[i] {
                    let cell = self.back[i];
                    queue!(
                        out,
                        MoveTo(x, y),
                        SetForegroundColor(cell.style.fg),
                        SetBackgroundColor(cell.style.bg),
                        Print(cell.ch)
                    )?;
                }
            }
        }
        out.flush()?;
        std::mem::swap(&mut self.front, &mut self.back);
        Ok(())
    }
}

/// Lifecycle hooks for the application
pub trait Application {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool;

    fn on_user_update(&mut self, ctx: &mut Context) -> bool;
}

/// Manages the application lifecycle, user input / display, etc.
/// while running the provided app.
pub struct ConsoleRunner<T: Application> {
    app: T,
    context: Context,
}

impl<T: Application> ConsoleRunner<T> {
    /// Begins application
    fn run(&mut self) {}
}
