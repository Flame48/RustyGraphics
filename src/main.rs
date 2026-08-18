#[allow(unused)]
use std::io;

mod application;
mod scene;

fn main() -> io::Result<()> {
    let mut runner = application::ConsoleRunner::new(scene::App::new())?;
    runner.run()
}
