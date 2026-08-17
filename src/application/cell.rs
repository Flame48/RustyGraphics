use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellStyle {
    pub fg: Color,
    pub bg: Color,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: CellStyle,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self { fg: Color::White, bg: Color::Reset }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', style: CellStyle::default() }
    }
}
