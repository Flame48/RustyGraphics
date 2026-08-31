pub mod cell;
pub mod context;

use std::{ io::{ self }, time::{ Duration, Instant } };
use crossterm::{
    cursor::{ Hide, Show },
    event::{ self, Event, KeyEvent },
    execute,
    terminal::{ EnterAlternateScreen, LeaveAlternateScreen },
};

use crate::application::Application;

pub use context::ConsoleRenderingContext2D;

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
pub struct ConsoleRunner<T: Application<ConsoleRenderingContext2D, KeyEvent>> {
    app: T,
    ctx: ConsoleRenderingContext2D,
}

impl<T: Application<ConsoleRenderingContext2D, KeyEvent>> ConsoleRunner<T> {
    pub fn new(app: T) -> io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(ConsoleRunner {
            app: app,
            ctx: ConsoleRenderingContext2D::new(width as usize, height as usize),
        })
    }

    fn is_open(&mut self) -> anyhow::Result<bool> {
        Ok(self.app.on_user_start(&mut self.ctx))
    }

    /// Begins application
    pub fn run(&mut self) -> anyhow::Result<()> {
        let _guard = TerminalGuard::new()?;

        if !self.is_open()? {
            return Ok(());
        }

        let mut last_time = Instant::now();

        loop {
            let events = self.poll_events()?;

            let current_time = Instant::now();
            let dt = (current_time - last_time).as_secs_f32();
            last_time = current_time;

            let mut should_quit = false;

            for event in events {
                match event {
                    Event::Key(k) => {
                        if !self.app.on_user_key_press(k, dt) {
                            should_quit = true;
                            break;
                        }
                    }
                    Event::Resize(new_w, new_h) => {
                        self.ctx.resize(new_w as usize, new_h as usize);
                    }
                    _ => {}
                }
            }

            if should_quit {
                break;
            }

            if !self.app.on_user_update(&mut self.ctx, dt) {
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
