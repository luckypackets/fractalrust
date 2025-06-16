//! # Fractal Generator
//!
//! A high-performance, terminal-based fractal generator written in Rust using the ratatui TUI framework.
//! Generate and explore beautiful fractals including Mandelbrot sets, Julia sets, Burning Ship,
//! Tricorn, and Multibrot fractals directly in your terminal.
//!
//! ## Features
//!
//! - Multiple fractal types (Mandelbrot, Julia, Burning Ship, Tricorn, Multibrot)
//! - Real-time interactive navigation with zoom and pan
//! - Auto-generation mode with smooth transitions
//! - Equation editor for custom fractal parameters
//! - High-performance multi-threaded computation
//! - Intelligent caching system
//! - Terminal-based UI with Unicode characters and colors
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use fractal_generator::App;
//! use ratatui::{backend::CrosstermBackend, Terminal};
//! use crossterm::{
//!     event::{DisableMouseCapture, EnableMouseCapture},
//!     execute,
//!     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//! };
//! use std::io;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Setup terminal
//!     enable_raw_mode()?;
//!     let mut stdout = io::stdout();
//!     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//!     let backend = CrosstermBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!
//!     // Create and run app
//!     let mut app = App::new();
//!     let res = app.run(&mut terminal);
//!
//!     // Restore terminal
//!     disable_raw_mode()?;
//!     execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
//!     terminal.show_cursor()?;
//!
//!     if let Err(err) = res {
//!         println!("Error: {:?}", err);
//!     }
//!
//!     Ok(())
//! }
//! ```

/// Main application logic and UI coordination
pub mod app;
/// Fractal generation algorithms and mathematical computations
pub mod fractal;
/// User interface components and layout
pub mod ui;
/// Terminal rendering and character/color mapping
pub mod renderer;
/// Configuration management and serialization
pub mod config;

pub use app::App;
pub use fractal::{FractalType, FractalParams, FractalGenerator};
pub use ui::UI;
pub use renderer::TerminalRenderer;
pub use config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_fractal_generator_creation() {
        let generator = FractalGenerator::new();
        assert!(generator.use_adaptive_sampling);
        assert!(!generator.performance_mode);
    }

    #[test]
    fn test_mandelbrot_basic_generation() {
        let generator = FractalGenerator::new();
        let params = FractalParams {
            fractal_type: FractalType::Mandelbrot,
            width: 10,
            height: 10,
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            max_iterations: 100,
        };

        let result = generator.generate(&params);
        assert_eq!(result.len(), 10);
        assert_eq!(result[0].len(), 10);

        // Check that we get different iteration counts (not all the same)
        let mut unique_values = std::collections::HashSet::new();
        for row in &result {
            for &value in row {
                unique_values.insert(value);
            }
        }
        assert!(unique_values.len() > 1, "Should have varying iteration counts");
    }

    #[test]
    fn test_julia_set_generation() {
        let generator = FractalGenerator::new();
        let params = FractalParams {
            fractal_type: FractalType::Julia { c: Complex::new(-0.7, 0.27) },
            width: 5,
            height: 5,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
            max_iterations: 50,
        };

        let result = generator.generate(&params);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 5);
    }

    #[test]
    fn test_burning_ship_generation() {
        let generator = FractalGenerator::new();
        let params = FractalParams {
            fractal_type: FractalType::BurningShip,
            width: 5,
            height: 5,
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            max_iterations: 50,
        };

        let result = generator.generate(&params);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 5);
    }

    #[test]
    fn test_performance_mode() {
        let mut generator = FractalGenerator::new();
        generator.set_performance_mode(true);
        assert!(generator.performance_mode);

        generator.set_performance_mode(false);
        assert!(!generator.performance_mode);
    }

    #[test]
    fn test_adaptive_sampling() {
        let mut generator = FractalGenerator::new();
        generator.set_adaptive_sampling(false);
        assert!(!generator.use_adaptive_sampling);

        generator.set_adaptive_sampling(true);
        assert!(generator.use_adaptive_sampling);
    }

    #[test]
    fn test_terminal_renderer_creation() {
        let renderer = TerminalRenderer::new();
        assert!(renderer.use_colors);
        assert!(renderer.use_unicode);
        assert!(!renderer.use_fast_rendering);
    }

    #[test]
    fn test_renderer_settings() {
        let mut renderer = TerminalRenderer::new();

        renderer.set_use_colors(false);
        assert!(!renderer.use_colors);

        renderer.set_use_unicode(false);
        assert!(!renderer.use_unicode);

        renderer.set_fast_rendering(true);
        assert!(renderer.use_fast_rendering);
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert!(config.display.use_colors);
        assert!(config.display.use_unicode);
        assert_eq!(config.display.default_width, 80);
        assert_eq!(config.display.default_height, 40);
        assert_eq!(config.fractal.default_zoom, 1.0);
        assert_eq!(config.fractal.default_max_iterations, 100);
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());

        let mut bad_config = Config::default();
        bad_config.display.default_width = 0;
        assert!(bad_config.validate().is_err());
    }

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(!app.should_quit);
        assert_eq!(app.zoom_factor, 1.0);
        assert_eq!(app.center_x, -0.5);
        assert_eq!(app.center_y, 0.0);
        assert_eq!(app.max_iterations, 100);
    }
}
