use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub struct TerminalRenderer {
    // Configuration for rendering
    pub use_colors: bool,
    pub use_unicode: bool,
    pub use_fast_rendering: bool,
    pub quality_mode: bool,
    pub super_sampling: bool,
    pub last_rendered_data: Option<Vec<Vec<u32>>>,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            use_colors: true,
            use_unicode: true,
            use_fast_rendering: false,
            quality_mode: true,
            super_sampling: false,
            last_rendered_data: None,
        }
    }

    pub fn set_fast_rendering(&mut self, enabled: bool) {
        self.use_fast_rendering = enabled;
    }

    pub fn set_quality_mode(&mut self, enabled: bool) {
        self.quality_mode = enabled;
    }

    pub fn set_super_sampling(&mut self, enabled: bool) {
        self.super_sampling = enabled;
    }

    pub fn render_to_text(&mut self, fractal_data: &[Vec<u32>], target_width: usize, target_height: usize) -> Vec<Line> {
        if fractal_data.is_empty() {
            return vec![Line::from("No fractal data")];
        }

        let data_height = fractal_data.len();
        let data_width = fractal_data[0].len();

        // Check if we can use differential rendering
        let use_differential = self.use_fast_rendering &&
            self.last_rendered_data.as_ref()
                .map(|last| last.len() == data_height && last[0].len() == data_width)
                .unwrap_or(false);

        // Scale the fractal data to fit the target dimensions
        let mut lines = Vec::new();

        for y in 0..target_height.min(data_height) {
            let mut spans = Vec::new();

            for x in 0..target_width.min(data_width) {
                let iterations = fractal_data[y][x];

                // Skip rendering if pixel hasn't changed (differential rendering)
                if use_differential {
                    if let Some(ref last_data) = self.last_rendered_data {
                        if last_data[y][x] == iterations {
                            // Use cached character for unchanged pixels
                            spans.push(Span::raw(" "));
                            continue;
                        }
                    }
                }

                let (character, color) = self.iterations_to_char_and_color(iterations);

                let span = if self.use_colors {
                    Span::styled(character.to_string(), Style::default().fg(color))
                } else {
                    Span::raw(character.to_string())
                };

                spans.push(span);
            }

            lines.push(Line::from(spans));
        }

        // Cache the current data for next differential render
        if self.use_fast_rendering {
            self.last_rendered_data = Some(fractal_data.to_vec());
        }

        lines
    }

    pub fn render_to_text_with_bounds(
        &mut self,
        fractal_data: &[Vec<u32>],
        start_x: usize,
        start_y: usize,
        display_width: usize,
        display_height: usize,
        target_width: usize,
        target_height: usize
    ) -> Vec<Line> {
        if fractal_data.is_empty() {
            return vec![Line::from("No fractal data")];
        }

        let data_height = fractal_data.len();
        let data_width = if data_height > 0 { fractal_data[0].len() } else { 0 };

        let mut lines = Vec::new();

        // Calculate centering offsets if the fractal is smaller than the target area
        let center_offset_x = if display_width < target_width { (target_width - display_width) / 2 } else { 0 };
        let center_offset_y = if display_height < target_height { (target_height - display_height) / 2 } else { 0 };

        for target_y in 0..target_height {
            let mut spans = Vec::new();

            for target_x in 0..target_width {
                let char_and_color = if target_y >= center_offset_y &&
                                       target_y < center_offset_y + display_height &&
                                       target_x >= center_offset_x &&
                                       target_x < center_offset_x + display_width {
                    // We're in the fractal display area
                    let fractal_x = start_x + (target_x - center_offset_x);
                    let fractal_y = start_y + (target_y - center_offset_y);

                    if fractal_y < data_height && fractal_x < data_width {
                        let iterations = fractal_data[fractal_y][fractal_x];
                        self.iterations_to_char_and_color(iterations)
                    } else {
                        (' ', Color::Black) // Outside fractal bounds
                    }
                } else {
                    // We're in the padding area
                    (' ', Color::Black)
                };

                let span = if self.use_colors {
                    Span::styled(char_and_color.0.to_string(), Style::default().fg(char_and_color.1))
                } else {
                    Span::raw(char_and_color.0.to_string())
                };

                spans.push(span);
            }

            lines.push(Line::from(spans));
        }

        lines
    }

    fn iterations_to_char_and_color(&self, iterations: u32) -> (char, Color) {
        if self.use_unicode {
            self.iterations_to_unicode_char_and_color(iterations)
        } else {
            self.iterations_to_ascii_char_and_color(iterations)
        }
    }

    fn iterations_to_unicode_char_and_color(&self, iterations: u32) -> (char, Color) {
        // Enhanced mapping with more detail for quality mode
        if self.quality_mode {
            self.iterations_to_high_quality_char_and_color(iterations)
        } else {
            self.iterations_to_standard_char_and_color(iterations)
        }
    }

    fn iterations_to_high_quality_char_and_color(&self, iterations: u32) -> (char, Color) {
        // High-quality mapping with fine gradations and more Unicode characters
        match iterations {
            0..=1 => (' ', Color::Black),           // Deep space - completely outside
            2..=3 => ('·', Color::DarkGray),        // Very far outside
            4..=5 => ('░', Color::DarkGray),        // Far outside
            6..=8 => ('▒', Color::Gray),            // Outside boundary
            9..=12 => ('▓', Color::LightBlue),      // Approaching boundary
            13..=16 => ('█', Color::Blue),          // Near boundary
            17..=20 => ('▉', Color::Cyan),          // Boundary region
            21..=25 => ('▊', Color::LightCyan),     // Close to set
            26..=30 => ('▋', Color::Green),         // Very close to set
            31..=35 => ('▌', Color::LightGreen),    // Entering interesting region
            36..=40 => ('▍', Color::Yellow),        // Interesting region
            41..=45 => ('▎', Color::LightYellow),   // Complex boundary
            46..=50 => ('▏', Color::Red),           // Near set boundary
            51..=60 => ('▕', Color::LightRed),      // At boundary
            61..=70 => ('▔', Color::Magenta),       // Edge of set
            71..=80 => ('▁', Color::LightMagenta),  // Deep boundary
            81..=90 => ('▂', Color::White),         // Very deep
            91..=100 => ('▃', Color::LightBlue),    // Deeper still
            101..=120 => ('▄', Color::Blue),        // Deep in set
            121..=140 => ('▅', Color::Cyan),        // Very deep
            141..=160 => ('▆', Color::Green),       // Extremely deep
            161..=180 => ('▇', Color::Yellow),      // Ultra deep
            181..=200 => ('█', Color::Red),         // Maximum depth
            201..=220 => ('▓', Color::Magenta),     // Beyond normal
            221..=240 => ('▒', Color::LightMagenta), // Infinite depth
            241..=255 => ('░', Color::White),       // Pure set
            _ => ('█', Color::LightMagenta),        // In the set
        }
    }

    fn iterations_to_standard_char_and_color(&self, iterations: u32) -> (char, Color) {
        // Standard mapping for performance mode
        match iterations {
            0..=2 => (' ', Color::Black),           // Very quick escape - far outside
            3..=5 => ('░', Color::DarkGray),        // Quick escape - outside
            6..=10 => ('▒', Color::Gray),           // Medium escape - boundary area
            11..=15 => ('▓', Color::White),         // Slower escape - near boundary
            16..=20 => ('█', Color::Blue),          // Even slower - interesting area
            21..=30 => ('█', Color::Cyan),          // Getting closer to set
            31..=40 => ('█', Color::LightGreen),    // Closer to set
            41..=50 => ('█', Color::Yellow),        // Even closer
            51..=60 => ('█', Color::LightYellow),   // Very close to set
            61..=70 => ('█', Color::Red),           // Near the set boundary
            71..=80 => ('█', Color::LightRed),      // At the boundary
            81..=90 => ('▓', Color::Magenta),       // Edge of set
            91..=99 => ('*', Color::LightRed),      // Almost in set
            _ => ('#', Color::LightMagenta),        // In the set
        }
    }

    fn iterations_to_ascii_char_and_color(&self, iterations: u32) -> (char, Color) {
        // Map iterations to ASCII characters and colors
        match iterations {
            0..=2 => (' ', Color::Black),           // Very quick escape
            3..=5 => ('.', Color::DarkGray),        // Quick escape
            6..=10 => (':', Color::Gray),           // Medium escape
            11..=15 => (';', Color::White),         // Slower escape
            16..=20 => ('!', Color::Blue),          // Even slower
            21..=30 => ('|', Color::Cyan),          // Getting closer
            31..=40 => ('$', Color::Green),         // Close to set
            41..=50 => ('@', Color::Yellow),        // Very close
            51..=70 => ('&', Color::Red),           // Near boundary
            71..=90 => ('%', Color::Magenta),       // Very near
            91..=99 => ('*', Color::LightRed),      // Almost in set
            _ => ('#', Color::LightMagenta),        // In the set
        }
    }

    pub fn set_use_colors(&mut self, use_colors: bool) {
        self.use_colors = use_colors;
    }

    pub fn set_use_unicode(&mut self, use_unicode: bool) {
        self.use_unicode = use_unicode;
    }

    // Method to render fractal data to a simple string (for debugging or text output)
    pub fn render_to_string(&self, fractal_data: &[Vec<u32>]) -> String {
        let mut result = String::new();
        
        for row in fractal_data {
            for &iterations in row {
                let (character, _) = self.iterations_to_char_and_color(iterations);
                result.push(character);
            }
            result.push('\n');
        }
        
        result
    }

    // Method to get color palette information
    pub fn get_color_info(&self) -> Vec<(String, Color)> {
        vec![
            ("Very Low (0-5)".to_string(), Color::Black),
            ("Low (6-15)".to_string(), Color::DarkGray),
            ("Medium-Low (16-25)".to_string(), Color::Blue),
            ("Medium (26-40)".to_string(), Color::Green),
            ("Medium-High (41-70)".to_string(), Color::Yellow),
            ("High (71-100)".to_string(), Color::Red),
            ("Very High (101-200)".to_string(), Color::Magenta),
            ("Extreme (200+)".to_string(), Color::White),
        ]
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}
