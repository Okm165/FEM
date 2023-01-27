use crate::{base_function::BaseFunction, fem::Problem};

pub const ZERO_TOLERANCE: f64 = 1e-8f64;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct GravitationalPotential;

impl Problem for GravitationalPotential {
    fn range(&self) -> std::ops::Range<f64> {
        0.0..3.0
    }

    fn left_integral<BFn: BaseFunction>(&self, x: f64, u: &BFn, v: &BFn) -> f64 {
        -1.0 * u.derivative(x) * v.derivative(x)
    }

    fn right_integral<BFn: BaseFunction>(&self, x: f64, v: &BFn) -> f64 {
        let p;
        if x < 1.0 {
            p = -0.5;
        } else if 1.0 < x && x < 2.0 {
            p = 1.0
        } else {
            p = 0.5
        }
        let f = 4.0 * PI * 1.0 * p;
        f * v.fun(x) - (2.0 / 3.0) * v.derivative(x)
    }

    fn evaluate(&self, x: f64, w: f64) -> f64 {
        9.0 - (2.0 / 3.0) * x + w
    }
}
