pub mod cell;
pub mod context;

use std::io::{ self, Write };
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ Color, Print, SetBackgroundColor, SetForegroundColor },
};

pub use cell::Cell;
pub use context::Context;

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
