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
use rand::Rng;

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
    pub status_message: String,
    pub show_help: bool,
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
            max_iterations: 100,
            current_equation: "z^2 + c".to_string(),
            status_message: "Ready".to_string(),
            show_help: false,
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
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key);
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
                self.status_message = "Switched to Auto-Generate mode".to_string();
            },
            KeyCode::Char('3') => {
                self.mode = AppMode::EquationEditor;
                self.input_mode = InputMode::Editing;
                self.status_message = "Equation Editor - Type new equation, Enter to apply, Esc to cancel".to_string();
            },
            KeyCode::Char('r') | KeyCode::F(5) => {
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
            _ => {}
        }
    }

    fn handle_editing_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
                self.mode = AppMode::Interactive;
                self.regenerate_fractal();
            },
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.mode = AppMode::Interactive;
            },
            KeyCode::Backspace => {
                self.current_equation.pop();
            },
            KeyCode::Char(c) => {
                self.current_equation.push(c);
            },
            _ => {}
        }
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            AppMode::AutoGenerate => {
                if self.auto_generation_timer.elapsed() > Duration::from_millis(2000) {
                    self.auto_update_parameters();
                    self.regenerate_fractal();
                    self.auto_generation_timer = Instant::now();
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn regenerate_fractal(&mut self) {
        let params = FractalParams {
            fractal_type: FractalType::Mandelbrot,
            width: 80,
            height: 24, // Reduced height to fit better in terminal
            zoom: self.zoom_factor,
            center_x: self.center_x,
            center_y: self.center_y,
            max_iterations: self.max_iterations,
        };

        self.fractal_data = self.fractal_generator.generate(&params);
        self.status_message = format!("Generated fractal - Zoom: {:.2}, Iterations: {}", self.zoom_factor, self.max_iterations);
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
        // Simple auto-generation: slowly zoom in and pan around
        let mut rng = rand::thread_rng();
        self.zoom_factor *= 1.1;
        self.center_x += (rng.gen::<f64>() - 0.5) * 0.01 / self.zoom_factor;
        self.center_y += (rng.gen::<f64>() - 0.5) * 0.01 / self.zoom_factor;

        // Reset if zoomed too far
        if self.zoom_factor > 1000.0 {
            self.zoom_factor = 1.0;
            self.center_x = -0.5;
            self.center_y = 0.0;
        }
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
                Constraint::Percentage(70), // Fractal display
                Constraint::Percentage(30), // Controls
            ])
            .split(area);

        self.render_fractal_display(f, main_chunks[0]);
        self.render_controls(f, main_chunks[1]);
    }

    fn render_fractal_display(&mut self, f: &mut Frame, area: Rect) {
        if self.fractal_data.is_empty() {
            self.regenerate_fractal();
        }

        let fractal_text = self.renderer.render_to_text(&self.fractal_data, area.width as usize, area.height as usize);
        
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

        let controls_text = format!(
            "Mode: {}{}\n\nParameters:\nZoom: {:.2}x\nCenter: ({:.3}, {:.3})\nIterations: {}\n\nEquation: {}\n\nControls:\n+/= : Zoom In\n-   : Zoom Out\n↑↓←→: Pan\ni   : More Iterations\nd   : Fewer Iterations\nr/Space: Regenerate\nc   : Reset Center\n1   : Interactive Mode\n2   : Auto Mode\n3   : Edit Equation\nh/F1: Toggle Help\nq/Esc: Quit",
            mode_str,
            input_indicator,
            self.zoom_factor,
            self.center_x,
            self.center_y,
            self.max_iterations,
            self.current_equation
        );

        let controls_widget = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(controls_widget, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer = Paragraph::new(self.status_message.as_str())
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
            - - Zoom out\n\n\
            Parameters:\n\
            i - Increase iterations\n\
            d - Decrease iterations\n\
            r - Regenerate fractal\n\n\
            General:\n\
            h - Toggle this help\n\
            q - Quit application\n\n\
            Press 'h' to close this help.";

        let help_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(help_widget, popup_area);
    }
}
