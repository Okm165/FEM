mod base_function;
mod fem;
mod integral;
mod models;

use base_function::TriangularBaseFunction;
use clap::Parser;
use plotters::{
    prelude::{full_palette::WHITE, BitMapBackend, ChartBuilder, IntoDrawingArea},
    series::LineSeries,
    style::BLUE,
};

use crate::{fem::FEMSolver, models::GravitationalPotential};

const CHART_POINTS: usize = 3000;

#[derive(Parser)]
/// Calulates GravitationalPotential problem with n finite elements and outputs it to out_path as png image with solution
struct Cli {
    /// number of elements
    #[arg(short, long)]
    n: usize,
    /// The path to the file to save output
    #[arg(short, long, default_value_t = String::from("output.png"))]
    out_path: String,
}

fn main() {
    let args = Cli::parse();

    let f = FEMSolver::solve::<TriangularBaseFunction, _>(&GravitationalPotential, args.n);

    let points: Vec<(f64, f64)> = (0..3 * CHART_POINTS)
        .map(|x| {
            (
                (x as f64) / (CHART_POINTS as f64),
                f.evalute((x as f64) / (CHART_POINTS as f64)),
            )
        })
        .collect();

    let backend = BitMapBackend::new(&args.out_path, (1024, 1024));
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0.5..3.5, -2.0..10.0)
        .unwrap();

    chart.draw_series(LineSeries::new(points, &BLUE)).unwrap();

    chart.configure_mesh().draw().unwrap();

    println!("Done file saved at {}", args.out_path);
}
