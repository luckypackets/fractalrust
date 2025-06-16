use num_complex::Complex;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia { c: Complex<f64> },
    BurningShip,
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
    // Future: Could add caching, optimization settings, etc.
}

impl FractalGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, params: &FractalParams) -> Vec<Vec<u32>> {
        match &params.fractal_type {
            FractalType::Mandelbrot => self.generate_mandelbrot(params),
            FractalType::Julia { c } => self.generate_julia(params, *c),
            FractalType::BurningShip => self.generate_burning_ship(params),
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
        let max_iterations = params.max_iterations;

        // Calculate the bounds of the complex plane to render
        let x_min = center_x - 2.0 / zoom;
        let x_max = center_x + 2.0 / zoom;
        let y_min = center_y - 2.0 / zoom;
        let y_max = center_y + 2.0 / zoom;

        let x_scale = (x_max - x_min) / width as f64;
        let y_scale = (y_max - y_min) / height as f64;

        // Generate fractal data using parallel processing
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
}

impl Default for FractalGenerator {
    fn default() -> Self {
        Self::new()
    }
}
