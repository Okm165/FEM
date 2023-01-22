mod tests;

pub trait BaseFunction {
    fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self;
    fn fun(&self, x: f64) -> f64;
    fn derivative(&self, x: f64) -> f64;
    fn non_zero_range(&self) -> std::ops::Range<f64>;
}

pub struct TriangularBaseFunction {
    k: usize,
    h: f64,
}

impl TriangularBaseFunction {
    fn get_points(&self) -> (f64, f64, f64) {
        let mid = (self.k as f64) * self.h;
        (mid - self.h, mid, mid + self.h)
    }
}

impl BaseFunction for TriangularBaseFunction {
    fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self {
        Self {
            k: index,
            h: (range.end - range.start) / (elements_count as f64),
        }
    }

    fn fun(&self, x: f64) -> f64 {
        let (low, mid, high) = self.get_points();
        let res: f64;
        if low <= x && x <= mid {
            res = (x - low) / self.h;
        } else if mid < x && x <= high {
            res = (high - x) / self.h;
        } else {
            res = 0.0
        }
        res
    }

    fn derivative(&self, x: f64) -> f64 {
        let (low, mid, high) = self.get_points();
        let res: f64;
        if low <= x && x <= mid {
            res = 1.0 / self.h;
        } else if mid < x && x <= high {
            res = 1.0 / -self.h;
        } else {
            res = 0.0;
        }
        res
    }

    fn non_zero_range(&self) -> std::ops::Range<f64> {
        let (low, _, high) = self.get_points();
        low..high
    }
}

impl std::fmt::Display for TriangularBaseFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (low, mid, high) = self.get_points();
        write!(
            f,
            "k: {}, h: {}, low: {}, mid: {}, high: {}",
            self.k, self.h, low, mid, high
        )
    }
}

pub fn range_intersection(ranges: &[std::ops::Range<f64>]) -> std::ops::Range<f64> {
    let mut low = f64::MIN;
    let mut high = f64::MAX;

    for range in ranges {
        low = low.max(range.start);
        high = high.min(range.end);
    }

    low..high.max(low)
}
