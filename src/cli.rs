#[allow(unused)]
use std::io;
use clap::Parser;
use graphics::{ application, scene };

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

    /// Optional texture to use for the color channels of the object surface during model inspection.
    /// By default, renders object with a white texture.
    #[arg(short, long, value_name = "FILE", requires = "inspect")]
    color_texture: Option<String>,

    /// This is an optional scaling factor to be applied to a model before it get's rendered.
    /// If omitted, performs auto-scaling
    #[arg(long, value_name = "FACTOR", requires = "inspect")]
    scale_factor: Option<f32>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = (
        if args.debug {
            scene::App::new_debug()
        } else if let Some(obj_path) = args.inspect {
            scene::App::new_obj_preview(obj_path, args.color_texture, args.scale_factor)
        } else {
            panic!("Invalid Arguments!")
        }
    ).expect("App failed to initialize");

    let mut runner = application::ConsoleRunner
        ::new(app)
        .expect("Unable to initialize application runner");

    runner.run()
}
