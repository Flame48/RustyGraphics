use std::env;
#[allow(unused)]
use std::io;
use anyhow::Error;
use graphics::{ application, scene };

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        return Err(Error::msg("Missing Argument!"));
    };

    let app = scene::App::new_obj_preview(path.clone(), None).expect("App failed to initialize");
    let mut runner = application::ConsoleRunner
        ::new(app)
        .expect("Unable to initialize application runner");
    runner.run()
}
