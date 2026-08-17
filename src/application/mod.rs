pub mod cell;
pub mod context;

use std::{ io::{ self }, time::Duration };
use crossterm::{
    cursor::{ Hide, Show },
    event::{ self, Event, KeyCode },
    execute,
    terminal::{ EnterAlternateScreen, LeaveAlternateScreen },
};

pub use cell::Cell;
pub use context::Context;

/// Lifecycle hooks for the application
pub trait Application {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool;

    fn on_user_update(&mut self, ctx: &mut Context) -> bool;
}

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

/// Manages the application lifecycle, user input / display, etc.
/// while running the provided app.
pub struct ConsoleRunner<T: Application> {
    app: T,
    ctx: Context,
}

impl<T: Application> ConsoleRunner<T> {
    pub fn new(app: T) -> io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(ConsoleRunner { app: app, ctx: Context::new(width, height) })
    }

    fn open(&mut self) -> io::Result<bool> {
        Ok(self.app.on_user_start(&mut self.ctx))
    }

    /// Begins application
    pub fn run(&mut self) -> io::Result<()> {
        let _guard = TerminalGuard::new()?;

        if !self.open()? {
            return Ok(());
        }

        loop {
            let events = self.poll_events()?;

            let mut should_quit = false;

            for event in events {
                match event {
                    Event::Key(k) => {
                        if k.code == KeyCode::Char('q') {
                            should_quit = true;
                            break;
                        }
                    }
                    Event::Resize(new_w, new_h) => {
                        self.ctx.resize(new_w, new_h);
                    }
                    _ => {}
                }
            }

            if should_quit {
                break;
            }

            if !self.app.on_user_update(&mut self.ctx) {
                break;
            }
            self.ctx.present(&mut io::stdout())?;
        }

        Ok(())
    }

    fn poll_events(&self) -> io::Result<Vec<Event>> {
        let mut events: Vec<Event> = Vec::default();
        while event::poll(Duration::ZERO)? {
            events.push(event::read()?);
        }
        Ok(events)
    }
}
