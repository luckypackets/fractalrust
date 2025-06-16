use fractal_generator::*;
use num_complex::Complex;

#[test]
fn test_full_fractal_generation_pipeline() {
    let generator = FractalGenerator::new();
    let mut renderer = TerminalRenderer::new();
    
    // Test Mandelbrot generation
    let params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        width: 20,
        height: 20,
        zoom: 1.0,
        center_x: -0.5,
        center_y: 0.0,
        max_iterations: 100,
    };
    
    let fractal_data = generator.generate(&params);
    let rendered_lines = renderer.render_to_text(&fractal_data, 20, 20);
    
    assert_eq!(rendered_lines.len(), 20);
    assert!(!rendered_lines.is_empty());
}

#[test]
fn test_performance_mode_affects_generation() {
    let mut generator = FractalGenerator::new();
    
    let params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        width: 10,
        height: 10,
        zoom: 1.0,
        center_x: -0.5,
        center_y: 0.0,
        max_iterations: 100,
    };
    
    // Generate without performance mode
    generator.set_performance_mode(false);
    let normal_result = generator.generate(&params);
    
    // Generate with performance mode
    generator.set_performance_mode(true);
    let performance_result = generator.generate(&params);
    
    // Both should have same dimensions
    assert_eq!(normal_result.len(), performance_result.len());
    assert_eq!(normal_result[0].len(), performance_result[0].len());
    
    // Results might be different due to reduced iterations in performance mode
    // but both should be valid
    assert!(!normal_result.is_empty());
    assert!(!performance_result.is_empty());
}

#[test]
fn test_adaptive_sampling_at_high_zoom() {
    let mut generator = FractalGenerator::new();
    
    let params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        width: 10,
        height: 10,
        zoom: 15.0, // High zoom to trigger adaptive sampling
        center_x: -0.5,
        center_y: 0.0,
        max_iterations: 100,
    };
    
    // Test with adaptive sampling
    generator.set_adaptive_sampling(true);
    let adaptive_result = generator.generate(&params);
    
    // Test without adaptive sampling
    generator.set_adaptive_sampling(false);
    let normal_result = generator.generate(&params);
    
    // Both should produce valid results
    assert_eq!(adaptive_result.len(), 10);
    assert_eq!(normal_result.len(), 10);
    assert_eq!(adaptive_result[0].len(), 10);
    assert_eq!(normal_result[0].len(), 10);
}

#[test]
fn test_different_fractal_types() {
    let generator = FractalGenerator::new();
    
    let base_params = FractalParams {
        fractal_type: FractalType::Mandelbrot, // Will be overridden
        width: 8,
        height: 8,
        zoom: 1.0,
        center_x: -0.5,
        center_y: 0.0,
        max_iterations: 50,
    };
    
    // Test Mandelbrot
    let mandelbrot_params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        ..base_params
    };
    let mandelbrot_result = generator.generate(&mandelbrot_params);
    assert_eq!(mandelbrot_result.len(), 8);
    
    // Test Julia Set
    let julia_params = FractalParams {
        fractal_type: FractalType::Julia { c: Complex::new(-0.7, 0.27) },
        ..base_params
    };
    let julia_result = generator.generate(&julia_params);
    assert_eq!(julia_result.len(), 8);
    
    // Test Burning Ship
    let burning_ship_params = FractalParams {
        fractal_type: FractalType::BurningShip,
        ..base_params
    };
    let burning_ship_result = generator.generate(&burning_ship_params);
    assert_eq!(burning_ship_result.len(), 8);
    
    // Test Tricorn
    let tricorn_params = FractalParams {
        fractal_type: FractalType::Tricorn,
        ..base_params
    };
    let tricorn_result = generator.generate(&tricorn_params);
    assert_eq!(tricorn_result.len(), 8);
    
    // Test Multibrot
    let multibrot_params = FractalParams {
        fractal_type: FractalType::Multibrot { power: 3.0 },
        ..base_params
    };
    let multibrot_result = generator.generate(&multibrot_params);
    assert_eq!(multibrot_result.len(), 8);
    
    // All results should be different (at least some values)
    assert_ne!(mandelbrot_result, julia_result);
    assert_ne!(mandelbrot_result, burning_ship_result);
}

#[test]
fn test_renderer_color_modes() {
    // Create some test fractal data
    let test_data = vec![
        vec![0, 10, 20, 50],
        vec![5, 15, 30, 100],
        vec![1, 25, 40, 150],
        vec![3, 35, 60, 200],
    ];

    // Test with colors
    let mut colored_renderer = TerminalRenderer::new();
    colored_renderer.set_use_colors(true);
    let colored_result = colored_renderer.render_to_text(&test_data, 4, 4);
    assert_eq!(colored_result.len(), 4);

    // Test without colors
    let mut uncolored_renderer = TerminalRenderer::new();
    uncolored_renderer.set_use_colors(false);
    let uncolored_result = uncolored_renderer.render_to_text(&test_data, 4, 4);
    assert_eq!(uncolored_result.len(), 4);

    // Both should produce valid output
    assert!(!colored_result.is_empty());
    assert!(!uncolored_result.is_empty());
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    
    // Test that config can be serialized to JSON
    let json_result = serde_json::to_string(&config);
    assert!(json_result.is_ok());
    
    // Test that it can be deserialized back
    let json_str = json_result.unwrap();
    let deserialized_result: Result<Config, _> = serde_json::from_str(&json_str);
    assert!(deserialized_result.is_ok());
    
    let deserialized_config = deserialized_result.unwrap();
    assert_eq!(config.display.use_colors, deserialized_config.display.use_colors);
    assert_eq!(config.fractal.default_zoom, deserialized_config.fractal.default_zoom);
}

#[test]
fn test_app_initialization() {
    let app = App::new();
    
    // Test initial state
    assert!(!app.should_quit);
    assert_eq!(app.zoom_factor, 1.0);
    assert_eq!(app.center_x, -0.5);
    assert_eq!(app.center_y, 0.0);
    assert_eq!(app.max_iterations, 100);
    assert!(app.fractal_cache.is_empty());
    assert_eq!(app.frame_count, 0);
    assert_eq!(app.fps, 0.0);
}

#[test]
fn test_zoom_and_pan_bounds() {
    let generator = FractalGenerator::new();
    
    // Test extreme zoom levels
    let high_zoom_params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        width: 5,
        height: 5,
        zoom: 1000.0,
        center_x: -0.5,
        center_y: 0.0,
        max_iterations: 50,
    };
    
    let result = generator.generate(&high_zoom_params);
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].len(), 5);
    
    // Test extreme pan positions
    let extreme_pan_params = FractalParams {
        fractal_type: FractalType::Mandelbrot,
        width: 5,
        height: 5,
        zoom: 1.0,
        center_x: 100.0,
        center_y: 100.0,
        max_iterations: 50,
    };
    
    let result2 = generator.generate(&extreme_pan_params);
    assert_eq!(result2.len(), 5);
    assert_eq!(result2[0].len(), 5);
}

#[test]
fn test_app_resize_handling() {
    use ratatui::layout::Rect;

    let mut app = App::new();

    // Test initial state
    assert_eq!(app.fractal_display_area, None);
    assert_eq!(app.last_terminal_size, None);

    // Simulate a resize event
    app.handle_resize_event(120, 40);

    // Check that the resize was recorded
    assert_eq!(app.last_terminal_size, Some((120, 40)));

    // Simulate setting a display area (this would normally happen during rendering)
    let test_area = Rect::new(0, 0, 80, 30);
    app.fractal_display_area = Some(test_area);

    // Generate a fractal with the current settings
    app.regenerate_fractal();

    // Verify that fractal data was generated
    assert!(!app.fractal_data.is_empty());

    // The fractal should be sized to fit the display area (minus borders)
    let expected_height = (test_area.height.saturating_sub(2) as usize).max(10);
    let expected_width = (test_area.width.saturating_sub(2) as usize).max(20);

    assert_eq!(app.fractal_data.len(), expected_height);
    if !app.fractal_data.is_empty() {
        assert_eq!(app.fractal_data[0].len(), expected_width);
    }
}

#[test]
fn test_renderer_with_bounds() {
    let mut renderer = TerminalRenderer::new();

    // Create a small test fractal
    let fractal_data = vec![
        vec![0, 10, 20, 30],
        vec![40, 50, 60, 70],
        vec![80, 90, 100, 110],
    ];

    // Test rendering with bounds and centering
    let lines = renderer.render_to_text_with_bounds(
        &fractal_data,
        0, 0,  // start_x, start_y
        4, 3,  // display_width, display_height
        6, 5   // target_width, target_height (larger to test centering)
    );

    // Should have 5 lines (target_height)
    assert_eq!(lines.len(), 5);

    // Each line should have 6 characters (target_width)
    for line in &lines {
        assert_eq!(line.spans.len(), 6);
    }
}
