#[allow(unused)]
use std::io;
use std::fs::OpenOptions;
use std::io::Write;

mod application;
mod scene;

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("debug.log") {
        let _ = writeln!(f, "{msg}");
    }
}

fn main() -> io::Result<()> {
    let app = scene::App::new().expect("App failed to initialize");
    let mut runner = application::ConsoleRunner::new(app)?;
    runner.run()
}
