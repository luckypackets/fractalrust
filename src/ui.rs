use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn render_loading_screen(f: &mut Frame, progress: f64, message: &str) {
        let area = f.size();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(area);

        let title = Paragraph::new("Fractal Generator")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[1]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Loading"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((progress * 100.0) as u16)
            .label(message);
        f.render_widget(gauge, chunks[2]);
    }

    pub fn render_error_popup(f: &mut Frame, error_message: &str) {
        let area = f.size();
        let popup_area = Self::centered_rect(60, 20, area);

        f.render_widget(Clear, popup_area);

        let error_widget = Paragraph::new(error_message)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .style(Style::default().fg(Color::Red))
            );
        f.render_widget(error_widget, popup_area);
    }

    pub fn render_info_popup(f: &mut Frame, title: &str, content: &str) {
        let area = f.size();
        let popup_area = Self::centered_rect(70, 60, area);

        f.render_widget(Clear, popup_area);

        let info_widget = Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().fg(Color::Cyan))
            );
        f.render_widget(info_widget, popup_area);
    }

    pub fn render_parameter_panel(
        f: &mut Frame,
        area: Rect,
        zoom: f64,
        center_x: f64,
        center_y: f64,
        iterations: u32,
        equation: &str,
        mode: &str,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);

        let mode_widget = Paragraph::new(format!("Mode: {}", mode))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(mode_widget, chunks[0]);

        let zoom_widget = Paragraph::new(format!("Zoom: {:.2}", zoom))
            .style(Style::default().fg(Color::Green));
        f.render_widget(zoom_widget, chunks[1]);

        let center_widget = Paragraph::new(format!("Center: ({:.3}, {:.3})", center_x, center_y))
            .style(Style::default().fg(Color::Blue));
        f.render_widget(center_widget, chunks[2]);

        let iterations_widget = Paragraph::new(format!("Iterations: {}", iterations))
            .style(Style::default().fg(Color::Magenta));
        f.render_widget(iterations_widget, chunks[3]);

        let equation_widget = Paragraph::new(format!("Equation: {}", equation))
            .style(Style::default().fg(Color::Cyan))
            .wrap(Wrap { trim: true });
        f.render_widget(equation_widget, chunks[4]);
    }

    pub fn render_controls_help(f: &mut Frame, area: Rect) {
        let help_items = vec![
            ListItem::new("Arrow Keys - Pan"),
            ListItem::new("+/= - Zoom In"),
            ListItem::new("- - Zoom Out"),
            ListItem::new("i - More Iterations"),
            ListItem::new("d - Fewer Iterations"),
            ListItem::new("r - Regenerate"),
            ListItem::new("1 - Interactive Mode"),
            ListItem::new("2 - Auto Mode"),
            ListItem::new("3 - Edit Equation"),
            ListItem::new("h - Toggle Help"),
            ListItem::new("q - Quit"),
        ];

        let help_list = List::new(help_items)
            .block(Block::default().borders(Borders::ALL).title("Controls"))
            .style(Style::default().fg(Color::White));
        f.render_widget(help_list, area);
    }

    pub fn render_color_legend(f: &mut Frame, area: Rect, color_info: &[(String, Color)]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); color_info.len()])
            .split(area);

        for (i, (description, color)) in color_info.iter().enumerate() {
            if i < chunks.len() {
                let legend_item = Paragraph::new(description.as_str())
                    .style(Style::default().fg(*color));
                f.render_widget(legend_item, chunks[i]);
            }
        }
    }

    pub fn render_status_bar(f: &mut Frame, area: Rect, status: &str, fps: Option<f64>) {
        let status_text = if let Some(fps) = fps {
            format!("{} | FPS: {:.1}", status, fps)
        } else {
            status.to_string()
        };

        let status_widget = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status_widget, area);
    }

    // Helper function to create a centered rectangle
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}
