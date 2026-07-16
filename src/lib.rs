use wasm_bindgen::prelude::*;
use mccga::mccga;

#[wasm_bindgen]
pub struct TestFunction {
    name: String,
    lowerbounds: Vec<f64>,
    upperbounds: Vec<f64>,
    costfn: fn(&Vec<f64>) -> f64,
    latexstring: String,
}

#[wasm_bindgen]
pub fn ackley() -> TestFunction {
    TestFunction {
    name: String::from("Ackley"),
    lowerbounds: vec![-32.768; 5],
    upperbounds: vec![32.768; 5],
    costfn: |x: &Vec<f64>| {
        let a = 20.0;
        let b = 0.2;
        let c = 2.0 * std::f64::consts::PI;
        let d = x.len() as f64;
        let sum1: f64 = x.iter().map(|&xi| xi * xi).sum();
        let sum2: f64 = x.iter().map(|&xi| (c * xi).cos()).sum();
        -a * (-b * (sum1 / d).sqrt()).exp() - (sum2 / d).exp() + a + std::f64::consts::E
    },
    latexstring: "f(x) = -a \\exp\\left(-b \\sqrt{\\frac{1}{d} \\sum_{i=1}^{d} x_i^2}\\right) - \\exp\\left(\\frac{1}{d} \\sum_{i=1}^{d} \\cos(c x_i)\\right) + a + e".to_string(),
    }
}

#[wasm_bindgen]
pub fn bohachevsky() -> TestFunction {
    TestFunction {
    name: String::from("Bohacevsky"),
    lowerbounds: vec![-100.0; 5],
    upperbounds: vec![100.0; 5],
    costfn: |x: &Vec<f64>| {
        let d = x.len() as f64;
        let sum1: f64 = x.iter().map(|&xi| xi * xi).sum();
        let sum2: f64 = x.iter().map(|&xi| (5.0 * xi).cos()).sum();
        sum1 - 0.1 * sum2 + 0.1 * d
    },
    latexstring: "f(x) = \\sum_{i=1}^{d} x_i^2 - 0.1 \\sum_{i=1}^{d} \\cos(5 x_i) + 0.1 d".to_string(),
    }
}

#[wasm_bindgen]
pub fn rastrigin() -> TestFunction {
    TestFunction {
    name: String::from("Rastrigin"),
    lowerbounds: vec![-5.12; 5],
    upperbounds: vec![5.12; 5],
    costfn: |x: &Vec<f64>| {
        let d = x.len() as f64;
        10.0 * d + x.iter().map(|&xi| xi * xi - 10.0 * (2.0 * std::f64::consts::PI * xi).cos()).sum::<f64>()
    },
    latexstring: "f(x) = 10 d + \\sum_{i=1}^{d} \\left(x_i^2 - 10 \\cos(2 \\pi x_i)\\right)".to_string(),
    }
}

#[wasm_bindgen]
pub fn rosenbrock() -> TestFunction {
    TestFunction {
    name: String::from("Rosenbrock"),
    lowerbounds: vec![-5.0; 5],
    upperbounds: vec![10.0; 5],
    costfn: |x: &Vec<f64>| {
        let mut sum = 0.0;
        for i in 0..(x.len() - 1) {
            sum += 100.0 * (x[i + 1] - x[i] * x[i]).powi(2) + (x[i] - 1.0).powi(2);
        }
        sum
    },
    latexstring: "f(x) = \\sum_{i=1}^{d-1} \\left[100 (x_{i+1} - x_i^2)^2 + (x_i - 1)^2\\right]".to_string(),
    }
}

#[wasm_bindgen]
pub fn sphere() -> TestFunction {
    TestFunction {
    name: String::from("Sphere"),
    lowerbounds: vec![-5.12; 5],
    upperbounds: vec![5.12; 5],
    costfn: |x: &Vec<f64>| {
        x.iter().map(|&xi| xi * xi).sum()
    },
    latexstring: "f(x) = \\sum_{i=1}^{d} x_i^2".to_string(),
    }
}

#[wasm_bindgen]
impl TestFunction {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn lowerbounds(&self) -> Vec<f64> {
        self.lowerbounds.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn upperbounds(&self) -> Vec<f64> {
        self.upperbounds.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn latexstring(&self) -> String {
        self.latexstring.clone()
    }

    #[wasm_bindgen]
    pub fn evaluate(&self, x: Vec<f64>) -> f64 {
        (self.costfn)(&x)
    }

    #[wasm_bindgen]
    pub fn optimize(&self) -> Vec<f64> {
        let result = mccga(
            self.costfn,
            &self.lowerbounds,
            &self.upperbounds,
            0.001, // mutrate
            10000, // number of generations
        );
        return result;
    }
}

#[wasm_bindgen]
pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello from Rust!");
}
