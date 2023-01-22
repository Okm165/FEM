mod legendre_table;
mod tests;

use legendre_table::LEGENDRE_TABLE;

pub fn integrate<Fu>(range: std::ops::Range<f64>, f: Fu) -> f64
where
    Fu: Fn(f64) -> f64,
{
    let mut sum = 0.0;
    for (w, x) in LEGENDRE_TABLE.iter() {
        sum += w * f(0.5 * (range.start + range.end) + 0.5 * x * (range.end - range.start));
    }
    (range.end - range.start) * sum * 0.5
}
