#[cfg(test)]
mod tests {
    use crate::{
        base_function::{BaseFunction, TriangularBaseFunction},
        models::ZERO_TOLERANCE,
    };

    #[test]
    fn test_triangular_base_function() {
        let bfn = TriangularBaseFunction::new(7, 10, 0.0..1.0);
        let (low, mid, high) = bfn.get_points();
        assert!(bfn.k == 7);
        assert!((bfn.h - 0.1).abs() < ZERO_TOLERANCE);
        assert!((low - 0.6).abs() < ZERO_TOLERANCE);
        assert!((mid - 0.7).abs() < ZERO_TOLERANCE);
        assert!((high - 0.8).abs() < ZERO_TOLERANCE);
    }
}
