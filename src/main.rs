#[allow(unused)]
use std::io;

mod application;
mod renderer;

fn main() -> io::Result<()> {
    let mut runner = application::ConsoleRunner::new(renderer::App)?;
    runner.run()
}
