use nalgebra::{DMatrix, DVector};

use crate::{
    base_function::{range_intersection, BaseFunction},
    integral::integrate,
    models::ZERO_TOLERANCE,
};

pub trait Problem {
    fn range(&self) -> std::ops::Range<f64>;

    fn left_integral<BFn: BaseFunction>(&self, x: f64, u: &BFn, v: &BFn) -> f64;

    fn right_integral<BFn: BaseFunction>(&self, x: f64, v: &BFn) -> f64;

    fn evaluate(&self, x: f64, w: f64) -> f64;
}

pub struct ComputedFunction<BFn: BaseFunction, P: Problem> {
    bases: Vec<BFn>,
    scalars: DVector<f64>,
    problem: P,
}

impl<BFn: BaseFunction, P: Problem> ComputedFunction<BFn, P> {
    pub fn evalute(&self, x: f64) -> f64
    where
        P: Problem,
    {
        self.problem.evaluate(
            x,
            self.bases
                .iter()
                .zip(self.scalars.iter())
                .map(|(e, w)| e.fun(x) * w)
                .sum(),
        )
    }
}

pub struct FEMSolver;

impl FEMSolver {
    pub fn gen_bases<BFn, P>(problem: &P, n: usize) -> Vec<BFn>
    where
        P: Problem,
        BFn: BaseFunction,
    {
        let mut bases = Vec::<BFn>::with_capacity(n);
        for i in 1..n {
            bases.push(BFn::new(i, n, problem.range()));
        }
        bases
    }

    pub fn solve<BFn, P>(problem: &P, n: usize) -> ComputedFunction<BFn, P>
    where
        P: Problem + Clone,
        BFn: BaseFunction,
    {
        let bases = Self::gen_bases::<BFn, P>(problem, n);

        let right = DVector::<f64>::from_fn(bases.len(), |row, _| {
            let v = &bases[row];
            let func = |x| problem.right_integral(x, v);
            let range = range_intersection(&[problem.range(), v.non_zero_range()]);

            if (range.end - range.start) > ZERO_TOLERANCE {
                integrate(range, func)
            } else {
                0.0
            }
        });

        let left = DMatrix::<f64>::from_fn(bases.len(), bases.len(), |row, col| {
            let v = &bases[row];
            let u = &bases[col];

            let func = |x| problem.left_integral(x, u, v);

            let range =
                range_intersection(&[problem.range(), v.non_zero_range(), u.non_zero_range()]);

            if (range.end - range.start) > ZERO_TOLERANCE {
                integrate(range, func)
            } else {
                0.0
            }
        });

        ComputedFunction {
            bases,
            scalars: left.lu().solve(&right).unwrap(),
            problem: problem.clone(),
        }
    }
}
