pub mod app;
pub mod fractal;
pub mod ui;
pub mod renderer;
pub mod config;

pub use app::App;
pub use fractal::{FractalType, FractalParams, FractalGenerator};
pub use ui::UI;
pub use renderer::TerminalRenderer;
pub use config::Config;
