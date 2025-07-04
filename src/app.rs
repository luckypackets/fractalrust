use crate::{FractalType, FractalParams, FractalGenerator, TerminalRenderer, Config};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use num_complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Interactive,
    AutoGenerate,
    EquationEditor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub should_quit: bool,
    pub mode: AppMode,
    pub input_mode: InputMode,
    pub fractal_generator: FractalGenerator,
    pub renderer: TerminalRenderer,
    pub config: Config,
    pub fractal_data: Vec<Vec<u32>>,
    pub auto_generation_timer: Instant,
    pub auto_generation_phase: u32,
    pub auto_target_zoom: f64,
    pub auto_target_x: f64,
    pub auto_target_y: f64,
    pub zoom_factor: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub max_iterations: u32,
    pub current_equation: String,
    pub current_fractal_type: FractalType,
    pub status_message: String,
    pub show_help: bool,
    pub fractal_cache: HashMap<String, Vec<Vec<u32>>>,
    pub last_render_time: Instant,
    pub frame_count: u32,
    pub fps: f64,
    pub total_generation_time: Duration,
    pub generation_count: u32,
    pub fractal_display_area: Option<Rect>,
    pub last_terminal_size: Option<(u16, u16)>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let config = Config::default();
        let fractal_generator = FractalGenerator::new();
        let renderer = TerminalRenderer::new();
        
        Self {
            should_quit: false,
            mode: AppMode::Interactive,
            input_mode: InputMode::Normal,
            fractal_generator,
            renderer,
            config,
            fractal_data: Vec::new(),
            auto_generation_timer: Instant::now(),
            auto_generation_phase: 0,
            auto_target_zoom: 1.0,
            auto_target_x: -0.5,
            auto_target_y: 0.0,
            zoom_factor: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            max_iterations: 256,
            current_equation: "z^2 + c".to_string(),
            current_fractal_type: FractalType::Mandelbrot,
            status_message: "Ready".to_string(),
            show_help: false,
            fractal_cache: HashMap::new(),
            last_render_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            total_generation_time: Duration::new(0, 0),
            generation_count: 0,
            fractal_display_area: None,
            last_terminal_size: None,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if self.should_quit {
                break;
            }

            self.handle_events()?;
            self.update()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    self.handle_key_event(key);
                }
                Event::Resize(width, height) => {
                    self.handle_resize_event(width, height);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_key_event(key),
            InputMode::Editing => self.handle_editing_key_event(key),
        }
    }

    pub fn handle_resize_event(&mut self, width: u16, height: u16) {
        let new_size = (width, height);

        // Only regenerate if the size actually changed
        if self.last_terminal_size != Some(new_size) {
            self.last_terminal_size = Some(new_size);

            // Clear the fractal display area so it gets recalculated
            self.fractal_display_area = None;

            // Clear cache since the fractal dimensions will change
            self.fractal_cache.clear();

            // The fractal will be regenerated on the next render when the new display area is calculated
            self.status_message = format!("Terminal resized to {}x{} - Regenerating fractal", width, height);
        }
    }

    fn handle_normal_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('h') | KeyCode::F(1) => self.show_help = !self.show_help,
            KeyCode::Char('1') => {
                self.mode = AppMode::Interactive;
                self.status_message = "Switched to Interactive mode".to_string();
            },
            KeyCode::Char('2') => {
                self.mode = AppMode::AutoGenerate;
                self.auto_generation_phase = 0;
                self.auto_generation_timer = Instant::now();
                self.status_message = "Switched to Auto-Generate mode - Exploring fractal automatically".to_string();
            },
            KeyCode::Char('3') => {
                self.mode = AppMode::EquationEditor;
                self.input_mode = InputMode::Editing;
                self.status_message = "Equation Editor - Type new equation, Enter to apply, Esc to cancel".to_string();
            },
            KeyCode::Char('r') => {
                self.regenerate_fractal();
                self.status_message = "Fractal regenerated".to_string();
            },
            KeyCode::Char('+') | KeyCode::Char('=') => self.zoom_in(),
            KeyCode::Char('-') | KeyCode::Char('_') => self.zoom_out(),
            KeyCode::Up => self.pan_up(),
            KeyCode::Down => self.pan_down(),
            KeyCode::Left => self.pan_left(),
            KeyCode::Right => self.pan_right(),
            KeyCode::Char('i') => self.increase_iterations(),
            KeyCode::Char('d') => self.decrease_iterations(),
            KeyCode::Char('c') => {
                // Reset to center
                self.center_x = -0.5;
                self.center_y = 0.0;
                self.zoom_factor = 1.0;
                self.status_message = "Reset to center view".to_string();
                self.regenerate_fractal();
            },
            KeyCode::Char(' ') => {
                // Spacebar to regenerate (alternative to 'r')
                self.regenerate_fractal();
                self.status_message = "Fractal regenerated".to_string();
            },
            KeyCode::F(2) => {
                // Quick preset: Burning Ship
                self.current_fractal_type = FractalType::BurningShip;
                self.current_equation = "Burning Ship".to_string();
                self.status_message = "Switched to Burning Ship fractal".to_string();
                self.regenerate_fractal();
            },
            KeyCode::F(3) => {
                // Quick preset: Julia Set
                self.current_fractal_type = FractalType::Julia { c: Complex::new(-0.7269, 0.1889) };
                self.current_equation = "Julia: c = -0.7269 + 0.1889i".to_string();
                self.status_message = "Switched to Julia Set fractal".to_string();
                self.regenerate_fractal();
            },
            KeyCode::F(4) => {
                // Quick preset: Tricorn
                self.current_fractal_type = FractalType::Tricorn;
                self.current_equation = "Tricorn".to_string();
                self.status_message = "Switched to Tricorn fractal".to_string();
                self.regenerate_fractal();
            },
            KeyCode::F(5) => {
                // Toggle performance mode
                self.fractal_generator.performance_mode = !self.fractal_generator.performance_mode;
                let mode_str = if self.fractal_generator.performance_mode { "ON" } else { "OFF" };
                self.status_message = format!("Performance mode: {}", mode_str);
                self.fractal_cache.clear(); // Clear cache when changing performance mode
                self.regenerate_fractal();
            },
            KeyCode::F(6) => {
                // Toggle adaptive sampling
                self.fractal_generator.use_adaptive_sampling = !self.fractal_generator.use_adaptive_sampling;
                let mode_str = if self.fractal_generator.use_adaptive_sampling { "ON" } else { "OFF" };
                self.status_message = format!("Adaptive sampling: {}", mode_str);
                self.fractal_cache.clear(); // Clear cache when changing sampling mode
                self.regenerate_fractal();
            },
            KeyCode::F(7) => {
                // Clear cache manually
                let cache_size = self.fractal_cache.len();
                self.fractal_cache.clear();
                self.status_message = format!("Cleared {} cached fractals", cache_size);
            },
            KeyCode::F(8) => {
                // Show performance statistics
                let avg_time = if self.generation_count > 0 {
                    self.total_generation_time.as_millis() / self.generation_count as u128
                } else {
                    0
                };
                self.status_message = format!(
                    "Performance: {} generations, avg {:.1}ms, FPS: {:.1}, Cache: {} entries",
                    self.generation_count, avg_time, self.fps, self.fractal_cache.len()
                );
            },
            KeyCode::F(9) => {
                let quality_mode = !self.fractal_generator.quality_mode;
                self.fractal_generator.set_quality_mode(quality_mode);
                self.renderer.set_quality_mode(quality_mode);
                self.fractal_cache.clear(); // Clear cache since quality changed
                self.status_message = format!("Quality Mode: {} - {}",
                    if quality_mode { "ON" } else { "OFF" },
                    if quality_mode { "Higher detail, more iterations" } else { "Standard detail" });
                self.regenerate_fractal();
            },
            KeyCode::F(10) => {
                let super_sampling = !self.fractal_generator.super_sampling;
                self.fractal_generator.set_super_sampling(super_sampling);
                self.renderer.set_super_sampling(super_sampling);
                self.fractal_cache.clear(); // Clear cache since sampling changed
                self.status_message = format!("Super Sampling: {} - {}",
                    if super_sampling { "ON" } else { "OFF" },
                    if super_sampling { "2x resolution (slower)" } else { "1x resolution (faster)" });
                self.regenerate_fractal();
            },
            _ => {}
        }
    }

    fn handle_editing_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if self.validate_and_apply_equation() {
                    self.input_mode = InputMode::Normal;
                    self.mode = AppMode::Interactive;
                    self.status_message = "Custom equation applied successfully".to_string();
                    self.regenerate_fractal();
                } else {
                    self.status_message = "Invalid equation format. Use supported patterns like 'z^2+c', 'z^3+c', etc.".to_string();
                }
            },
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.mode = AppMode::Interactive;
                self.status_message = "Equation editing cancelled".to_string();
            },
            KeyCode::Backspace => {
                self.current_equation.pop();
            },
            KeyCode::Char(c) => {
                if self.current_equation.len() < 50 { // Limit equation length
                    self.current_equation.push(c);
                }
            },
            _ => {}
        }
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            AppMode::AutoGenerate => {
                if self.auto_generation_timer.elapsed() > Duration::from_millis(100) {
                    self.auto_update_parameters();
                    self.regenerate_fractal();
                    self.auto_generation_timer = Instant::now();
                }
            },
            _ => {
                // Optimize parameters based on performance in interactive mode
                if self.last_render_time.elapsed() > Duration::from_secs(2) {
                    self.optimize_parameters_for_performance();
                }
            }
        }
        Ok(())
    }

    pub fn regenerate_fractal(&mut self) {
        self.regenerate_fractal_with_size(None);
    }

    fn regenerate_fractal_with_size(&mut self, size_override: Option<(usize, usize)>) {
        let start_time = Instant::now();

        // Use the fractal display area if available, otherwise fall back to defaults
        let (width, height) = if let Some((w, h)) = size_override {
            (w, h)
        } else if let Some(area) = self.fractal_display_area {
            // Account for borders (subtract 2 for left/right borders, 2 for top/bottom borders)
            let display_width = (area.width.saturating_sub(2) as usize).max(20);
            let display_height = (area.height.saturating_sub(2) as usize).max(10);
            (display_width, display_height)
        } else {
            // Default fallback
            (80, 24)
        };

        let params = FractalParams {
            fractal_type: self.current_fractal_type.clone(),
            width,
            height,
            zoom: self.zoom_factor,
            center_x: self.center_x,
            center_y: self.center_y,
            max_iterations: self.max_iterations,
        };

        // Create cache key
        let cache_key = self.create_cache_key(&params);

        // Check cache first
        if let Some(cached_data) = self.fractal_cache.get(&cache_key) {
            self.fractal_data = cached_data.clone();
            let generation_time = start_time.elapsed();
            self.status_message = format!("Cached fractal - Zoom: {:.2}, Iterations: {}, Time: {:.1}ms",
                self.zoom_factor, self.max_iterations, generation_time.as_millis());
        } else {
            // Generate new fractal
            self.fractal_data = self.fractal_generator.generate(&params);

            // Cache the result (limit cache size)
            if self.fractal_cache.len() < 50 {
                self.fractal_cache.insert(cache_key, self.fractal_data.clone());
            } else if self.fractal_cache.len() >= 100 {
                // Clear old cache entries when it gets too large
                self.fractal_cache.clear();
            }

            let generation_time = start_time.elapsed();
            self.total_generation_time += generation_time;
            self.generation_count += 1;

            let avg_time = self.total_generation_time.as_millis() / self.generation_count as u128;
            self.status_message = format!("Generated fractal - Zoom: {:.2}, Iterations: {}, Time: {:.1}ms (Avg: {:.1}ms)",
                self.zoom_factor, self.max_iterations, generation_time.as_millis(), avg_time);
        }

        // Update FPS counter
        self.update_fps();
    }

    fn zoom_in(&mut self) {
        self.zoom_factor *= 1.5;
        self.status_message = format!("Zoomed in to {:.2}x", self.zoom_factor);
        self.regenerate_fractal();
    }

    fn zoom_out(&mut self) {
        self.zoom_factor /= 1.5;
        self.status_message = format!("Zoomed out to {:.2}x", self.zoom_factor);
        self.regenerate_fractal();
    }

    fn pan_up(&mut self) {
        self.center_y -= 0.1 / self.zoom_factor;
        self.status_message = format!("Panned to ({:.3}, {:.3})", self.center_x, self.center_y);
        self.regenerate_fractal();
    }

    fn pan_down(&mut self) {
        self.center_y += 0.1 / self.zoom_factor;
        self.status_message = format!("Panned to ({:.3}, {:.3})", self.center_x, self.center_y);
        self.regenerate_fractal();
    }

    fn pan_left(&mut self) {
        self.center_x -= 0.1 / self.zoom_factor;
        self.status_message = format!("Panned to ({:.3}, {:.3})", self.center_x, self.center_y);
        self.regenerate_fractal();
    }

    fn pan_right(&mut self) {
        self.center_x += 0.1 / self.zoom_factor;
        self.status_message = format!("Panned to ({:.3}, {:.3})", self.center_x, self.center_y);
        self.regenerate_fractal();
    }

    fn increase_iterations(&mut self) {
        self.max_iterations = (self.max_iterations + 10).min(1000);
        self.status_message = format!("Increased iterations to {}", self.max_iterations);
        self.regenerate_fractal();
    }

    fn decrease_iterations(&mut self) {
        self.max_iterations = (self.max_iterations.saturating_sub(10)).max(10);
        self.status_message = format!("Decreased iterations to {}", self.max_iterations);
        self.regenerate_fractal();
    }

    fn auto_update_parameters(&mut self) {
        let mut rng = rand::thread_rng();

        match self.auto_generation_phase {
            // Phase 0: Explore the main bulb
            0..=200 => {
                self.auto_target_x = -0.5 + (rng.gen::<f64>() - 0.5) * 0.3;
                self.auto_target_y = (rng.gen::<f64>() - 0.5) * 0.3;
                self.auto_target_zoom = 1.0 + (self.auto_generation_phase as f64 / 50.0);
            },
            // Phase 1: Zoom into interesting areas
            201..=400 => {
                if self.auto_generation_phase == 201 {
                    // Pick an interesting point
                    let interesting_points = [
                        (-0.7269, 0.1889),   // Spiral
                        (-0.8, 0.156),       // Seahorse valley
                        (-0.74529, 0.11307), // Lightning
                        (-0.1, 0.651),       // Rabbit ears
                        (-0.75, 0.0),        // Needle point
                    ];
                    let point = interesting_points[rng.gen_range(0..interesting_points.len())];
                    self.auto_target_x = point.0;
                    self.auto_target_y = point.1;
                }
                self.auto_target_zoom = 5.0 + ((self.auto_generation_phase - 200) as f64 / 20.0);
            },
            // Phase 2: Deep zoom
            401..=600 => {
                self.auto_target_zoom = 15.0 + ((self.auto_generation_phase - 400) as f64 / 10.0);
                // Small random movements
                self.auto_target_x += (rng.gen::<f64>() - 0.5) * 0.001;
                self.auto_target_y += (rng.gen::<f64>() - 0.5) * 0.001;
            },
            // Phase 3: Switch fractal type and reset
            _ => {
                self.auto_generation_phase = 0;
                self.auto_target_x = -0.5;
                self.auto_target_y = 0.0;
                self.auto_target_zoom = 1.0;
                self.max_iterations = 100;

                // Cycle through different fractal types
                self.current_fractal_type = match &self.current_fractal_type {
                    FractalType::Mandelbrot => {
                        self.current_equation = "Burning Ship".to_string();
                        FractalType::BurningShip
                    },
                    FractalType::BurningShip => {
                        self.current_equation = "Julia Set".to_string();
                        FractalType::Julia { c: Complex::new(-0.7269, 0.1889) }
                    },
                    FractalType::Julia { .. } => {
                        self.current_equation = "Tricorn".to_string();
                        FractalType::Tricorn
                    },
                    FractalType::Tricorn => {
                        self.current_equation = "Multibrot z^3".to_string();
                        FractalType::Multibrot { power: 3.0 }
                    },
                    FractalType::Multibrot { .. } => {
                        self.current_equation = "z^2 + c".to_string();
                        FractalType::Mandelbrot
                    },
                    _ => {
                        self.current_equation = "z^2 + c".to_string();
                        FractalType::Mandelbrot
                    }
                };
                return;
            }
        }

        // Smooth interpolation towards targets
        let lerp_factor = 0.05;
        self.center_x += (self.auto_target_x - self.center_x) * lerp_factor;
        self.center_y += (self.auto_target_y - self.center_y) * lerp_factor;
        self.zoom_factor += (self.auto_target_zoom - self.zoom_factor) * lerp_factor;

        // Gradually increase iterations for better detail at high zoom
        if self.zoom_factor > 10.0 {
            self.max_iterations = (100 + (self.zoom_factor * 2.0) as u32).min(500);
        }

        self.auto_generation_phase += 1;

        let fractal_name = match &self.current_fractal_type {
            FractalType::Mandelbrot => "Mandelbrot",
            FractalType::BurningShip => "Burning Ship",
            FractalType::Julia { .. } => "Julia Set",
            FractalType::Tricorn => "Tricorn",
            FractalType::Multibrot { .. } => "Multibrot",
            _ => "Custom"
        };

        self.status_message = format!(
            "Auto-exploring {} - Phase {} - Zoom: {:.1}x, Iterations: {}",
            fractal_name,
            self.auto_generation_phase / 200,
            self.zoom_factor,
            self.max_iterations
        );
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ])
            .split(f.size());

        self.render_header(f, chunks[0]);
        self.render_main_content(f, chunks[1]);
        self.render_footer(f, chunks[2]);

        if self.show_help {
            self.render_help_popup(f);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let title = match self.mode {
            AppMode::Interactive => "Fractal Generator - Interactive Mode",
            AppMode::AutoGenerate => "Fractal Generator - Auto Generation Mode",
            AppMode::EquationEditor => "Fractal Generator - Equation Editor",
        };

        let header = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, area);
    }

    fn render_main_content(&mut self, f: &mut Frame, area: Rect) {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(80), // Fractal display
                Constraint::Percentage(20), // Controls
            ])
            .split(area);

        self.render_fractal_display(f, main_chunks[0]);
        self.render_controls(f, main_chunks[1]);
    }

    fn render_fractal_display(&mut self, f: &mut Frame, area: Rect) {
        // Store the display area for fractal generation
        let area_changed = self.fractal_display_area.map_or(true, |old_area| {
            old_area.width != area.width || old_area.height != area.height
        });

        if area_changed {
            self.fractal_display_area = Some(area);
        }

        // Regenerate fractal if empty or if the display area changed
        if self.fractal_data.is_empty() || area_changed {
            self.regenerate_fractal();
        }

        // Calculate the actual content area (inside the border)
        let content_width = area.width.saturating_sub(2) as usize;
        let content_height = area.height.saturating_sub(2) as usize;

        // Get the fractal dimensions
        let fractal_height = self.fractal_data.len();
        let fractal_width = if fractal_height > 0 { self.fractal_data[0].len() } else { 0 };

        // Calculate how much of the fractal to display and centering
        let display_width = content_width.min(fractal_width);
        let display_height = content_height.min(fractal_height);

        // Center the fractal if the display area is larger
        let start_x = if fractal_width < content_width { 0 } else { (fractal_width - content_width) / 2 };
        let start_y = if fractal_height < content_height { 0 } else { (fractal_height - content_height) / 2 };

        let fractal_text = self.renderer.render_to_text_with_bounds(
            &self.fractal_data,
            start_x, start_y,
            display_width, display_height,
            content_width, content_height
        );

        let fractal_widget = Paragraph::new(fractal_text)
            .block(Block::default().borders(Borders::ALL).title("Fractal"));
        f.render_widget(fractal_widget, area);
    }

    fn render_controls(&self, f: &mut Frame, area: Rect) {
        let mode_str = match self.mode {
            AppMode::Interactive => "Interactive",
            AppMode::AutoGenerate => "Auto-Generate",
            AppMode::EquationEditor => "Equation Editor",
        };

        let input_indicator = match self.input_mode {
            InputMode::Normal => "",
            InputMode::Editing => " [EDITING]",
        };

        let controls_text = if self.mode == AppMode::EquationEditor {
            format!(
                "Mode: {}{}\n\nEquation Editor:\nCurrent: {}\n\nExamples:\n• z^2 + c (Mandelbrot)\n• z^3 + c (Multibrot)\n• burning ship\n• tricorn\n• julia(-0.7, 0.27)\n\nControls:\nType equation\nEnter: Apply\nEsc: Cancel\n\nParameters:\nZoom: {:.2}x\nCenter: ({:.3}, {:.3})\nIterations: {}",
                mode_str,
                input_indicator,
                self.current_equation,
                self.zoom_factor,
                self.center_x,
                self.center_y,
                self.max_iterations
            )
        } else {
            format!(
                "Mode: {}{}\n\nParameters:\nZoom: {:.2}x\nCenter: ({:.3}, {:.3})\nIterations: {}\n\nEquation: {}\n\nQuality:\nQuality Mode: {}\nSuper Sampling: {}\n\nControls:\n+/= : Zoom In\n-   : Zoom Out\n↑↓←→: Pan\ni   : More Iterations\nd   : Fewer Iterations\nr/Space: Regenerate\nc   : Reset Center\n1   : Interactive Mode\n2   : Auto Mode\n3   : Edit Equation\nF9  : Quality Mode\nF10 : Super Sampling\nh/F1: Toggle Help\nq/Esc: Quit",
                mode_str,
                input_indicator,
                self.zoom_factor,
                self.center_x,
                self.center_y,
                self.max_iterations,
                self.current_equation,
                if self.fractal_generator.quality_mode { "ON" } else { "OFF" },
                if self.fractal_generator.super_sampling { "ON" } else { "OFF" }
            )
        };

        let controls_widget = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(controls_widget, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_text = if self.fps > 0.0 {
            format!("{} | FPS: {:.1} | Cache: {} entries",
                self.status_message, self.fps, self.fractal_cache.len())
        } else {
            self.status_message.clone()
        };

        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, area);
    }

    fn render_help_popup(&self, f: &mut Frame) {
        let area = f.size();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        f.render_widget(Clear, popup_area);

        let help_text = "FRACTAL GENERATOR HELP\n\n\
            Modes:\n\
            1 - Interactive Mode\n\
            2 - Auto Generation Mode\n\
            3 - Equation Editor\n\n\
            Navigation:\n\
            Arrow Keys - Pan around\n\
            +/= - Zoom in\n\
            - - Zoom out\n\
            c - Reset to center\n\n\
            Parameters:\n\
            i - Increase iterations\n\
            d - Decrease iterations\n\
            r/Space - Regenerate fractal\n\n\
            Quick Presets:\n\
            F2 - Burning Ship\n\
            F3 - Julia Set\n\
            F4 - Tricorn\n\n\
            Performance & Quality:\n\
            F5 - Toggle Performance Mode\n\
            F6 - Toggle Adaptive Sampling\n\
            F7 - Clear Cache\n\
            F8 - Show Performance Stats\n\
            F9 - Toggle Quality Mode\n\
            F10 - Toggle Super Sampling\n\n\
            Equation Editor:\n\
            Examples: z^3+c, burning ship,\n\
            tricorn, julia(-0.7, 0.27)\n\n\
            General:\n\
            h/F1 - Toggle this help\n\
            q/Esc - Quit application\n\n\
            Press 'h' to close this help.";

        let help_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(help_widget, popup_area);
    }

    fn validate_and_apply_equation(&mut self) -> bool {
        let equation = self.current_equation.trim().to_lowercase();

        // Parse and validate common fractal equation patterns
        if equation.is_empty() {
            return false;
        }

        // Try to parse the equation and set the appropriate fractal type
        if equation == "z^2+c" || equation == "z^2 + c" || equation == "mandelbrot" {
            self.current_fractal_type = FractalType::Mandelbrot;
            self.current_equation = "z^2 + c".to_string();
            return true;
        }

        if equation == "burning ship" || equation == "burningship" {
            self.current_fractal_type = FractalType::BurningShip;
            self.current_equation = "Burning Ship".to_string();
            return true;
        }

        if equation == "tricorn" {
            self.current_fractal_type = FractalType::Tricorn;
            self.current_equation = "Tricorn".to_string();
            return true;
        }

        // Parse z^n + c patterns (simple parsing without regex for now)
        if let Some(power) = self.parse_power_equation(&equation) {
            if power >= 2.0 && power <= 10.0 {
                self.current_fractal_type = FractalType::Multibrot { power };
                self.current_equation = format!("z^{} + c", power);
                return true;
            }
        }

        // Parse Julia set patterns
        if let Some((real, imag)) = self.parse_julia_equation(&equation) {
            self.current_fractal_type = FractalType::Julia { c: Complex::new(real, imag) };
            self.current_equation = format!("Julia: c = {} + {}i", real, imag);
            return true;
        }

        false
    }

    fn parse_power_equation(&self, equation: &str) -> Option<f64> {
        // Simple parsing for patterns like "z^3+c", "z^4 + c", etc.
        if equation.starts_with("z^") && equation.ends_with("+c") {
            let power_part = &equation[2..equation.len()-2].trim();
            if let Ok(power) = power_part.parse::<f64>() {
                return Some(power);
            }
        }
        if equation.starts_with("z^") && equation.ends_with("+ c") {
            let power_part = &equation[2..equation.len()-3].trim();
            if let Ok(power) = power_part.parse::<f64>() {
                return Some(power);
            }
        }
        None
    }

    fn parse_julia_equation(&self, equation: &str) -> Option<(f64, f64)> {
        // Simple parsing for patterns like "julia(-0.7, 0.27)"
        if equation.starts_with("julia(") && equation.ends_with(")") {
            let inner = &equation[6..equation.len()-1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(real), Ok(imag)) = (
                    parts[0].trim().parse::<f64>(),
                    parts[1].trim().parse::<f64>()
                ) {
                    return Some((real, imag));
                }
            }
        }
        None
    }

    fn create_cache_key(&self, params: &FractalParams) -> String {
        format!(
            "{:?}_{}_{}_{:.6}_{:.6}_{:.3}_{}",
            params.fractal_type,
            params.width,
            params.height,
            params.center_x,
            params.center_y,
            params.zoom,
            params.max_iterations
        )
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_render_time.elapsed();

        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_render_time = Instant::now();
        }
    }

    fn optimize_parameters_for_performance(&mut self) {
        // Adaptive quality based on zoom level and performance
        if self.fps < 10.0 && self.fps > 0.0 {
            // Performance is poor, reduce quality
            if self.max_iterations > 50 {
                self.max_iterations = (self.max_iterations * 9 / 10).max(50);
            }
        } else if self.fps > 30.0 {
            // Performance is good, can increase quality
            if self.max_iterations < 200 && self.zoom_factor > 5.0 {
                self.max_iterations = (self.max_iterations * 11 / 10).min(200);
            }
        }
    }
}
