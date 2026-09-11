#[allow(unused)]
use std::io;
use clap::Parser;

mod application;
mod scene;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(clap::ArgGroup::new("mode").required(true).args(["debug", "inspect"])))]
struct Args {
    /// Flag for debug scenes. This is used in development to setup custom scenes for testing.
    /// This is subject to change.
    #[arg(short, long)]
    debug: bool,

    /// Inspect mode for inspecting obj files. The files will be imported as a mesh into a basic
    /// scene with a camera and point light. The renderer will currently make use of a simple
    /// diffusion lighting model. The obj file will also spin slowly about the world y axis.
    #[arg(short, long, value_name = "FILE")]
    inspect: Option<String>,

    /// This is an optional scaling factor to be applied to a model before it get's rendered.
    #[arg(long, value_name = "FACTOR", requires = "inspect")]
    scale_factor: Option<f32>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let app = scene::App::new(args).expect("App failed to initialize");
    let mut runner = application::ConsoleRunner
        ::new(app)
        .expect("Unable to initialize application runner");
    runner.run()
}
