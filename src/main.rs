#[allow(unused)]
use std::io;
use std::io::Error;

mod application;
mod scene;

fn main() -> io::Result<()> {
    let app = scene::App::new().expect("App failed to initialize");
    let mut runner = application::ConsoleRunner::new(app)?;
    runner.run()
}
