use crate::application::{ Cell, Application, Context, cell::CellStyle };

mod math;
mod scene;

pub struct App;

impl Application for App {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool {
        ctx.clear();
        true
    }

    fn on_user_update(&mut self, ctx: &mut Context) -> bool {
        const BACK: Cell = Cell {
            ch: '.',
            style: CellStyle {
                fg: crossterm::style::Color::DarkGrey,
                bg: crossterm::style::Color::Reset,
            },
        };
        const LINE: Cell = Cell {
            ch: '#',
            style: CellStyle {
                fg: crossterm::style::Color::Cyan,
                bg: crossterm::style::Color::Reset,
            },
        };

        ctx.clear();
        ctx.fill(BACK);
        ctx.line(LINE, 20, 1, 5, 10);
        ctx.line(LINE, 20, 1, 35, 10);
        ctx.line(LINE, 5, 10, 35, 10);
        true
    }
}
