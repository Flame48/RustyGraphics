// Traits and Utils
pub mod application;
pub use application::Application;
mod buffer;

// Supported Application Types
pub mod console;
pub use console::ConsoleRunner;

pub mod window;
pub use window::WindowRunner;
