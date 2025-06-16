use num_complex::Complex;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia { c: Complex<f64> },
    BurningShip,
    Tricorn,
    Multibrot { power: f64 },
    Custom { equation: String },
}

#[derive(Debug, Clone)]
pub struct FractalParams {
    pub fractal_type: FractalType,
    pub width: usize,
    pub height: usize,
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub max_iterations: u32,
}

pub struct FractalGenerator {
    pub use_adaptive_sampling: bool,
    pub performance_mode: bool,
}

impl FractalGenerator {
    pub fn new() -> Self {
        Self {
            use_adaptive_sampling: true,
            performance_mode: false,
        }
    }

    pub fn set_performance_mode(&mut self, enabled: bool) {
        self.performance_mode = enabled;
    }

    pub fn set_adaptive_sampling(&mut self, enabled: bool) {
        self.use_adaptive_sampling = enabled;
    }

    pub fn generate(&self, params: &FractalParams) -> Vec<Vec<u32>> {
        match &params.fractal_type {
            FractalType::Mandelbrot => self.generate_mandelbrot(params),
            FractalType::Julia { c } => self.generate_julia(params, *c),
            FractalType::BurningShip => self.generate_burning_ship(params),
            FractalType::Tricorn => self.generate_tricorn(params),
            FractalType::Multibrot { power } => self.generate_multibrot(params, *power),
            FractalType::Custom { equation: _ } => {
                // For now, fallback to Mandelbrot for custom equations
                // TODO: Implement equation parser
                self.generate_mandelbrot(params)
            }
        }
    }

    fn generate_mandelbrot(&self, params: &FractalParams) -> Vec<Vec<u32>> {
        let width = params.width;
        let height = params.height;
        let zoom = params.zoom;
        let center_x = params.center_x;
        let center_y = params.center_y;
        let max_iterations = if self.performance_mode {
            (params.max_iterations / 2).max(20)
        } else {
            params.max_iterations
        };

        // Calculate the bounds of the complex plane to render
        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        // Use adaptive sampling for better performance at high zoom levels
        if self.use_adaptive_sampling && zoom > 10.0 {
            self.generate_mandelbrot_adaptive(width, height, x_min, x_max, y_min, y_max, max_iterations)
        } else {
            // Standard generation using parallel processing
            (0..height)
                .into_par_iter()
                .map(|y| {
                    (0..width)
                        .map(|x| {
                            let real = x_min + x as f64 * x_scale;
                            let imag = y_min + y as f64 * y_scale;
                            let c = Complex::new(real, imag);
                            self.mandelbrot_iterations(c, max_iterations)
                        })
                        .collect()
                })
                .collect()
        }
    }

    fn generate_mandelbrot_adaptive(&self, width: usize, height: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64, max_iterations: u32) -> Vec<Vec<u32>> {
        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        // First pass: sample every 2nd pixel
        let mut result = vec![vec![0u32; width]; height];

        (0..height)
            .into_par_iter()
            .step_by(2)
            .for_each(|y| {
                for x in (0..width).step_by(2) {
                    let real = x_min + x as f64 * x_scale;
                    let imag = y_min + y as f64 * y_scale;
                    let c = Complex::new(real, imag);
                    let iterations = self.mandelbrot_iterations(c, max_iterations);

                    // Fill 2x2 block with the same value for performance
                    if y < height {
                        result[y][x] = iterations;
                        if x + 1 < width {
                            result[y][x + 1] = iterations;
                        }
                    }
                    if y + 1 < height {
                        result[y + 1][x] = iterations;
                        if x + 1 < width {
                            result[y + 1][x + 1] = iterations;
                        }
                    }
                }
            });

        result
    }

    fn generate_julia(&self, params: &FractalParams, c: Complex<f64>) -> Vec<Vec<u32>> {
        let width = params.width;
        let height = params.height;
        let zoom = params.zoom;
        let center_x = params.center_x;
        let center_y = params.center_y;
        let max_iterations = params.max_iterations;

        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        (0..height)
            .into_par_iter()
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let real = x_min + x as f64 * x_scale;
                        let imag = y_min + y as f64 * y_scale;
                        let z = Complex::new(real, imag);
                        self.julia_iterations(z, c, max_iterations)
                    })
                    .collect()
            })
            .collect()
    }

    fn generate_burning_ship(&self, params: &FractalParams) -> Vec<Vec<u32>> {
        let width = params.width;
        let height = params.height;
        let zoom = params.zoom;
        let center_x = params.center_x;
        let center_y = params.center_y;
        let max_iterations = params.max_iterations;

        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        (0..height)
            .into_par_iter()
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let real = x_min + x as f64 * x_scale;
                        let imag = y_min + y as f64 * y_scale;
                        let c = Complex::new(real, imag);
                        self.burning_ship_iterations(c, max_iterations)
                    })
                    .collect()
            })
            .collect()
    }

    fn mandelbrot_iterations(&self, c: Complex<f64>, max_iterations: u32) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < max_iterations && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }

    fn julia_iterations(&self, mut z: Complex<f64>, c: Complex<f64>, max_iterations: u32) -> u32 {
        let mut iterations = 0;

        while iterations < max_iterations && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }

    fn burning_ship_iterations(&self, c: Complex<f64>, max_iterations: u32) -> u32 {
        let mut z: Complex<f64> = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < max_iterations && z.norm_sqr() <= 4.0 {
            // Burning ship: z = (|Re(z)| + i|Im(z)|)^2 + c
            let re_abs = z.re.abs();
            let im_abs = z.im.abs();
            z = Complex::new(re_abs, im_abs);
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }

    fn generate_tricorn(&self, params: &FractalParams) -> Vec<Vec<u32>> {
        let width = params.width;
        let height = params.height;
        let zoom = params.zoom;
        let center_x = params.center_x;
        let center_y = params.center_y;
        let max_iterations = params.max_iterations;

        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        (0..height)
            .into_par_iter()
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let real = x_min + x as f64 * x_scale;
                        let imag = y_min + y as f64 * y_scale;
                        let c = Complex::new(real, imag);
                        self.tricorn_iterations(c, max_iterations)
                    })
                    .collect()
            })
            .collect()
    }

    fn generate_multibrot(&self, params: &FractalParams, power: f64) -> Vec<Vec<u32>> {
        let width = params.width;
        let height = params.height;
        let zoom = params.zoom;
        let center_x = params.center_x;
        let center_y = params.center_y;
        let max_iterations = params.max_iterations;

        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        (0..height)
            .into_par_iter()
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let real = x_min + x as f64 * x_scale;
                        let imag = y_min + y as f64 * y_scale;
                        let c = Complex::new(real, imag);
                        self.multibrot_iterations(c, power, max_iterations)
                    })
                    .collect()
            })
            .collect()
    }

    fn tricorn_iterations(&self, c: Complex<f64>, max_iterations: u32) -> u32 {
        let mut z: Complex<f64> = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < max_iterations && z.norm_sqr() <= 4.0 {
            // Tricorn: z = conj(z)^2 + c
            z = z.conj();
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }

    fn multibrot_iterations(&self, c: Complex<f64>, power: f64, max_iterations: u32) -> u32 {
        let mut z: Complex<f64> = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < max_iterations && z.norm_sqr() <= 4.0 {
            // Multibrot: z = z^power + c
            z = z.powf(power) + c;
            iterations += 1;
        }

        iterations
    }
}

impl Default for FractalGenerator {
    fn default() -> Self {
        Self::new()
    }
}
