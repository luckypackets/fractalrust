# fractalrust

A high-performance, terminal-based fractal generator written in Rust using the ratatui TUI framework. Generate and explore beautiful fractals including Mandelbrot sets, Julia sets, Burning Ship, Tricorn, and Multibrot fractals directly in your terminal.

## Features

### 🎨 Multiple Fractal Types
- **Mandelbrot Set** - The classic fractal with infinite complexity
- **Julia Sets** - Beautiful filled Julia sets with customizable parameters
- **Burning Ship** - A variation of the Mandelbrot set with absolute values
- **Tricorn** - The "Mandelbar" set using complex conjugates
- **Multibrot** - Generalized Mandelbrot sets with custom powers (z^n + c)

### 🖥️ Terminal-Based Interface
- **Real-time rendering** using Unicode block characters and colors
- **Interactive navigation** with keyboard controls
- **Multiple display modes** - Interactive, Auto-generation, and Equation Editor
- **Responsive UI** that adapts to terminal size
- **Dynamic resize handling** - Fractal automatically adjusts when terminal is resized
- **Smart centering** - Fractal stays centered with padding when terminal is larger than needed
- **Optimized layout** - 80% screen space dedicated to fractal display for maximum detail

### ⚡ High Performance
- **Multi-threaded computation** using Rayon for parallel processing
- **Intelligent caching** system to avoid recomputation
- **Adaptive sampling** for better performance at high zoom levels
- **Performance monitoring** with FPS counter and timing statistics
- **Memory optimization** with efficient data structures

### 🎮 Interactive Controls
- **Zoom and Pan** - Explore fractals at any scale and position
- **Real-time parameter adjustment** - Modify iterations, zoom, and center point
- **Auto-generation mode** - Automatic exploration with smooth transitions
- **Equation editor** - Input custom fractal equations and parameters
- **Quick presets** - Function keys for instant fractal switching

## Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- A terminal that supports Unicode and colors

### Building from Source
```bash
git clone https://github.com/luckypackets/fractalrust.git
cd fractalrust
cargo build --release
```

### Running
```bash
cargo run --release
```

## Usage

### Basic Controls

#### Navigation
- **Arrow Keys** - Pan around the fractal
- **+/=** - Zoom in
- **-** - Zoom out
- **c** - Reset to center view

#### Parameters
- **i** - Increase iterations (more detail)
- **d** - Decrease iterations (faster rendering)
- **r/Space** - Regenerate fractal

#### Modes
- **1** - Interactive Mode (manual control)
- **2** - Auto-Generation Mode (automatic exploration)
- **3** - Equation Editor Mode (custom equations)

#### Quick Presets
- **F2** - Burning Ship fractal
- **F3** - Julia Set fractal
- **F4** - Tricorn fractal

#### Performance & Quality
- **F5** - Toggle Performance Mode (faster rendering)
- **F6** - Toggle Adaptive Sampling
- **F7** - Clear fractal cache
- **F8** - Show performance statistics
- **F9** - Toggle Quality Mode (higher detail)
- **F10** - Toggle Super Sampling (2x resolution)

#### General
- **h/F1** - Toggle help display
- **q/Esc** - Quit application

### Terminal Resize Support

The fractal generator now includes intelligent terminal resize handling:

- **Automatic Detection** - The application automatically detects when the terminal is resized
- **Dynamic Regeneration** - Fractals are regenerated to match the new terminal dimensions
- **Consistent Size** - The fractal maintains its logical size and mathematical accuracy
- **Smart Centering** - When the terminal is larger than needed, the fractal is centered with padding
- **Performance Optimized** - Resize events are debounced to avoid excessive regeneration

**How it works:**
- When you maximize your terminal or change its size, the fractal will automatically adjust
- The fractal computation uses the available display area efficiently
- No manual intervention required - just resize your terminal and continue exploring!

### Equation Editor

The equation editor supports several formats:

#### Basic Fractals
- `z^2 + c` or `mandelbrot` - Standard Mandelbrot set
- `burning ship` - Burning Ship fractal
- `tricorn` - Tricorn fractal

#### Power Variations
- `z^3 + c` - Cubic Mandelbrot (Multibrot)
- `z^4 + c` - Quartic Mandelbrot
- `z^n + c` - Any power from 2 to 10

#### Julia Sets
- `julia(-0.7, 0.27)` - Julia set with c = -0.7 + 0.27i
- `julia(real, imag)` - Custom Julia set parameters

### Auto-Generation Mode

Auto-generation mode automatically explores fractals with:
- **Phase-based exploration** - Different exploration patterns
- **Smooth transitions** - Interpolated movement between points
- **Fractal type cycling** - Automatically switches between different fractals
- **Adaptive parameters** - Adjusts quality based on zoom level

## Configuration

The application supports configuration through a JSON file:

```json
{
  "display": {
    "use_colors": true,
    "use_unicode": true,
    "default_width": 80,
    "default_height": 40
  },
  "fractal": {
    "default_zoom": 1.0,
    "default_center_x": -0.5,
    "default_center_y": 0.0,
    "default_max_iterations": 100
  },
  "performance": {
    "use_parallel_processing": true,
    "enable_caching": true,
    "max_cache_size": 100
  }
}
```

## Performance Tips

### For Better Performance
- Use **Performance Mode** (F5) for faster rendering at the cost of some detail
- Enable **Adaptive Sampling** (F6) for better performance at high zoom levels
- Reduce iterations with 'd' key when exploring new areas
- Clear cache (F7) if memory usage becomes high

### For Better Quality
- **Enable Quality Mode** (F9) for enhanced detail and higher iterations
- **Enable Super Sampling** (F10) for 2x resolution rendering
- Increase iterations with 'i' key for more detail
- Disable Performance Mode for full-quality rendering
- Use higher zoom levels to see fine fractal details
- Try different fractal types to see various mathematical structures

### Quality Features
- **Quality Mode** - Uses enhanced character mapping with 18+ gradation levels and higher iteration counts
- **Super Sampling** - Renders at 2x resolution then downsamples for smoother edges
- **Enhanced Color Palette** - More detailed color gradations for better visual distinction
- **Higher Default Iterations** - Increased from 100 to 256 for more detail by default

## Architecture

The application is structured into several modules:

- **`app.rs`** - Main application logic and UI coordination
- **`fractal.rs`** - Fractal generation algorithms and mathematical computations
- **`renderer.rs`** - Terminal rendering and character/color mapping
- **`ui.rs`** - User interface components and layout
- **`config.rs`** - Configuration management and serialization

### Key Design Principles
- **Performance First** - Multi-threaded computation with intelligent caching
- **Modular Design** - Separate concerns for easy maintenance and testing
- **User Experience** - Intuitive controls and real-time feedback
- **Extensibility** - Easy to add new fractal types and rendering modes

## Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# All tests with output
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses [rayon](https://github.com/rayon-rs/rayon) for parallel processing
- Mathematical computations powered by [num-complex](https://github.com/rust-num/num-complex)
- Inspired by the mathematical beauty of fractals and the power of Rust

## Screenshots

```
┌────────────────────────────────────────────────────────────────────────────────┐
│Fractal Generator - Interactive Mode                                           │
└────────────────────────────────────────────────────────────────────────────────┘
┌Fractal──────────────────────────────────────────────────┐┌Controls─────────────┐
│                                                         ││Mode: Interactive    │
│                    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░         ││                     │
│               ░░░░░░░░░░░░░░░░░░░░░░░▒▒▓▒▓█░░░░░░░░░     ││Parameters:          │
│           ░░░░░░░░░░░░░░░░░░░░░░░▒▒▒██████▒░░░░░░░░░░░   ││Zoom: 1.00x          │
│        ░░░░░░░░░░░░░░░░░░░░░▒█▓▓▒▓███████▓█▓█▒▒▒▓▒░░░░░ ││Center: (-0.500,0.000│
│       ░░░░░░░░░░░░░░░░░░▒▒▒▒▓▓███████████████████▒▒░░░░ ││Iterations: 100      │
│     ░░░░░░░░░░░░▒▒█▓█▒▓▓▒▒▒▒▒████████████████████████▒░ ││                     │
│     ░░░░░░░░░░▒▒▒▒█████████████████████████████████▓▒░░ ││Equation: z^2 + c    │
│    ██████████████████████████████████████████████▓▒▒▒░ ││                     │
└─────────────────────────────────────────────────────────┘└─────────────────────┘
┌────────────────────────────────────────────────────────────────────────────────┐
│Generated fractal - Zoom: 1.00, Iterations: 100, Time: 45ms | FPS: 22.1        │
└────────────────────────────────────────────────────────────────────────────────┘
```

Experience the beauty of mathematics in your terminal!
