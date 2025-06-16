use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub fractal: FractalConfig,
    pub performance: PerformanceConfig,
    pub controls: ControlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub use_colors: bool,
    pub use_unicode: bool,
    pub default_width: usize,
    pub default_height: usize,
    pub color_scheme: String,
    pub quality_mode: bool,
    pub super_sampling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalConfig {
    pub default_zoom: f64,
    pub default_center_x: f64,
    pub default_center_y: f64,
    pub default_max_iterations: u32,
    pub auto_generation_interval_ms: u64,
    pub zoom_step: f64,
    pub pan_step: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub use_parallel_processing: bool,
    pub thread_count: Option<usize>,
    pub enable_caching: bool,
    pub max_cache_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub zoom_in_key: String,
    pub zoom_out_key: String,
    pub pan_speed: f64,
    pub zoom_speed: f64,
    pub iteration_step: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                use_colors: true,
                use_unicode: true,
                default_width: 80,
                default_height: 40,
                color_scheme: "default".to_string(),
                quality_mode: true,
                super_sampling: false,
            },
            fractal: FractalConfig {
                default_zoom: 1.0,
                default_center_x: -0.5,
                default_center_y: 0.0,
                default_max_iterations: 256,
                auto_generation_interval_ms: 2000,
                zoom_step: 1.5,
                pan_step: 0.1,
            },
            performance: PerformanceConfig {
                use_parallel_processing: true,
                thread_count: None, // Use default (number of CPU cores)
                enable_caching: true,
                max_cache_size: 100,
            },
            controls: ControlsConfig {
                zoom_in_key: "+".to_string(),
                zoom_out_key: "-".to_string(),
                pan_speed: 1.0,
                zoom_speed: 1.0,
                iteration_step: 10,
            },
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(path).unwrap_or_default()
    }

    // Validation methods
    pub fn validate(&self) -> Result<(), String> {
        if self.display.default_width == 0 || self.display.default_height == 0 {
            return Err("Display dimensions must be greater than 0".to_string());
        }

        if self.fractal.default_max_iterations == 0 {
            return Err("Max iterations must be greater than 0".to_string());
        }

        if self.fractal.zoom_step <= 0.0 {
            return Err("Zoom step must be positive".to_string());
        }

        if self.fractal.auto_generation_interval_ms == 0 {
            return Err("Auto generation interval must be greater than 0".to_string());
        }

        if let Some(thread_count) = self.performance.thread_count {
            if thread_count == 0 {
                return Err("Thread count must be greater than 0".to_string());
            }
        }

        Ok(())
    }

    // Convenience methods for accessing commonly used values
    pub fn get_display_size(&self) -> (usize, usize) {
        (self.display.default_width, self.display.default_height)
    }

    pub fn get_default_fractal_params(&self) -> (f64, f64, f64, u32) {
        (
            self.fractal.default_zoom,
            self.fractal.default_center_x,
            self.fractal.default_center_y,
            self.fractal.default_max_iterations,
        )
    }

    pub fn should_use_colors(&self) -> bool {
        self.display.use_colors
    }

    pub fn should_use_unicode(&self) -> bool {
        self.display.use_unicode
    }

    pub fn get_auto_generation_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.fractal.auto_generation_interval_ms)
    }

    // Methods to update configuration at runtime
    pub fn set_display_colors(&mut self, use_colors: bool) {
        self.display.use_colors = use_colors;
    }

    pub fn set_display_unicode(&mut self, use_unicode: bool) {
        self.display.use_unicode = use_unicode;
    }

    pub fn set_fractal_defaults(&mut self, zoom: f64, center_x: f64, center_y: f64, max_iterations: u32) {
        self.fractal.default_zoom = zoom;
        self.fractal.default_center_x = center_x;
        self.fractal.default_center_y = center_y;
        self.fractal.default_max_iterations = max_iterations;
    }

    pub fn toggle_parallel_processing(&mut self) {
        self.performance.use_parallel_processing = !self.performance.use_parallel_processing;
    }
}
