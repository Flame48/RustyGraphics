use crate::application::{ Cell, Application, Context, cell::CellStyle };

mod math;

pub struct App;

impl Application for App {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool {
        ctx.clear();
        true
    }

    fn on_user_update(&mut self, ctx: &mut Context) -> bool {
        ctx.clear();
        ctx.fill(Cell {
            ch: '.',
            style: CellStyle {
                fg: crossterm::style::Color::DarkGrey,
                bg: crossterm::style::Color::Reset,
            },
        });
        true
    }
}
