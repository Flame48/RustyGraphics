#[allow(unused)]
mod application;

struct Renderer3D;

impl application::Application for Renderer3D {
    fn on_user_start(&mut self, _ctx: &mut application::Context) -> bool {
        true
    }

    fn on_user_update(&mut self, _ctx: &mut application::Context) -> bool {
        true
    }
}

fn main() {
    println!("Hello, world!");
}
