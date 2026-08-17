#[allow(unused)]
mod application;

struct Renderer3D;

impl application::Application for Renderer3D {
    fn on_user_start(&self) -> bool {
        return true;
    }

    fn on_user_update(&self) -> bool {
        return true;
    }
}

fn main() {
    println!("Hello, world!");
}
