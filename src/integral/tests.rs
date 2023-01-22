#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{integral::integrate, models::ZERO_TOLERANCE};

    #[test]
    fn test_intergate() {
        let val = integrate(-4.0..12.0, |x| x.powi(2));
        assert!((val - 597.333333333).abs() < ZERO_TOLERANCE);
    }

    #[test]
    fn test_intergate2() {
        let val = integrate(-PI..PI + 1.0, |x| x.powi(5));
        assert!((val - 680.878938294).abs() < ZERO_TOLERANCE);
    }
}
