pub const PKG_NAME: &str = "Pocus";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod cli;
mod gui;
mod functionality;

pub use functionality::*;
pub use cli::*;
pub use gui::*;