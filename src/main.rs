/*
sqrt(x^2 + y^2) = 0.96x + 0.28y

thats a unique strange solution that is always about + or - 4% accurate
lets find those points through brute force

sqrt(x^2 + y^2) = a*x + b*y
 */

use itertools_num::linspace;
use kdam::tqdm;
use rand::Rng;
use std::fmt;

fn f(x: f64, y: f64) -> f64 {
    // sqrt(x^2 + y^2)
    return (x.powi(2) + y.powi(2)).sqrt();
}
fn f_approximation(x: f64, y: f64, a: f64, b: f64) -> f64 {
    //  a*x + b*y
    return a * x + b * y;
}

#[derive(Clone)]
struct ErrorStats {
    total_error: f64,
    max_error: f64,
    min_error: f64,
    max_percent_error: f64,
    min_percent_error: f64,
}
impl Default for ErrorStats {
    fn default() -> Self {
        Self {
            total_error: 0.0,
            max_error: f64::NAN,
            min_error: f64::NAN,
            max_percent_error: f64::NAN,
            min_percent_error: f64::NAN,
        }
    }
}
impl fmt::Display for ErrorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "total_error: {:.2} max_error: {:.2} min_error: {:.2} max_percent_error: {:.2} min_percent_error: {:.2}",
            self.total_error, self.max_error, self.min_error, self.max_percent_error, self.min_percent_error
        )
    }
}

fn eval(
    a: f64,
    b: f64,

    iter_min: f64,
    iter_max: f64,
    iter_step: usize,

    f: fn(f64, f64) -> f64,
    approx_f: fn(f64, f64, f64, f64) -> f64,
) -> ErrorStats {
    //  evaluate the approximation function across a given range of values in all dimensions
    let mut error_stats: ErrorStats = ErrorStats::default();
    for x in linspace::<f64>(iter_min, iter_max, iter_step) {
        for y in linspace::<f64>(iter_min, iter_max, iter_step) {
            let true_output = f(x, y);
            let approx_output = approx_f(x, y, a, b);
            if true_output == 0.0 {
                continue;
                // true_output = 0.00000001;
            }
            let error = true_output - approx_output;
            let percent_error = approx_output / true_output;

            error_stats.total_error += error.abs();

            error_stats.max_error = error.max(error_stats.max_error);
            error_stats.min_error = error.min(error_stats.min_error);
            error_stats.max_percent_error = percent_error.max(error_stats.max_percent_error);
            error_stats.min_percent_error = percent_error.min(error_stats.min_percent_error);
        }
    }
    error_stats.max_percent_error *= 100.0;
    error_stats.min_percent_error *= 100.0;

    return error_stats;
}

fn print_best_error_stats(best_error_stats: &ErrorStats, total_iters: usize) {
    let avg_error = best_error_stats.total_error / total_iters as f64;

    println!("avg error: {:.3}", avg_error);
    println!(
        "lowest p error: {:.3}% / highest p error: {:.3}%",
        best_error_stats.min_percent_error, best_error_stats.max_percent_error
    );
    println!(
        "lowest error: {:.3} / highest error: {:.3}",
        best_error_stats.min_error, best_error_stats.max_error
    );
}

#[allow(dead_code)]
fn brute_force_search() {
    let mut best_a = 0.0;
    let mut best_b = 0.0;
    let mut best_error_stats: ErrorStats = ErrorStats::default();

    const PARAM_MIN: f64 = -1.0;
    const PARAM_MAX: f64 = 1.0;
    const PARAM_STEPS: usize = 100;

    const ITER_MIN: f64 = -1.0;
    const ITER_MAX: f64 = 1.0;
    const ITER_STEPS: usize = 100;

    for a in tqdm!(linspace::<f64>(PARAM_MIN, PARAM_MAX, PARAM_STEPS)) {
        for b in linspace::<f64>(PARAM_MIN, PARAM_MAX, PARAM_STEPS) {
            let error_stats = eval(a, b, ITER_MIN, ITER_MAX, ITER_STEPS, f, f_approximation);

            if best_error_stats.total_error == 0.0
                || (error_stats.total_error < best_error_stats.total_error)
            {
                best_a = a;
                best_b = b;
                best_error_stats = error_stats.clone();
            }
        }
    }

    println!("\n\nbest a and b: {:.3}, {:.3}", best_a, best_b);
    print_best_error_stats(&best_error_stats, ITER_STEPS * ITER_STEPS);
}

#[allow(dead_code)]
fn monty_carlo_descent() {
    let mut best_error_stats: ErrorStats = ErrorStats::default();

    const PARAM_MIN: f64 = -1.0;
    const PARAM_MAX: f64 = 1.0;
    const PARAM_STEPS: usize = 20;
    const PARAM_SUB_STEPS: usize = 100;

    const ITER_MIN: f64 = -1.0;
    const ITER_MAX: f64 = 1.0;
    const ITER_STEPS: usize = 100;

    let mut delta: f64 = (PARAM_MAX.abs() + PARAM_MIN.abs()) / 2.0;

    let mut a: f64 = PARAM_MAX + PARAM_MIN / 2.0;
    let mut b: f64 = PARAM_MAX + PARAM_MIN / 2.0;

    for _ in tqdm!(0..PARAM_STEPS) {
        for _ in 0..PARAM_SUB_STEPS {
            // add random number to a and b
            let mut rng = rand::thread_rng();
            let this_a = a + rng.gen_range(-delta..=delta);
            let this_b = b + rng.gen_range(-delta..=delta);
            let error_stats = eval(
                this_a,
                this_b,
                ITER_MIN,
                ITER_MAX,
                ITER_STEPS,
                f,
                f_approximation,
            );

            if best_error_stats.total_error == 0.0
                || (error_stats.total_error < best_error_stats.total_error)
            {
                // error is the same every time, it doesnt make sense
                a = this_b;
                b = this_b;
                best_error_stats = error_stats.clone();
                println!("new best error: {:.3}", best_error_stats.total_error);
                println!("new best a and b: {:.3}, {:.3}", this_a, this_b);
            }
        }
        delta /= 2.0;
    }

    println!("\n\nbest a and b: {:.3}, {:.3}", a, b);
    print_best_error_stats(&best_error_stats, ITER_STEPS * ITER_STEPS);
}

fn main() {
    // brute_force_search();
    monty_carlo_descent();
}

// use plotters::prelude::*;
// const OUT_FILE_NAME: &'static str = "3d-plot.svg";
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let area = SVGBackend::new(OUT_FILE_NAME, (1024, 760)).into_drawing_area();

//     area.fill(&WHITE)?;

//     let x_axis = (-3.0..3.0).step(0.1);
//     let z_axis = (-3.0..3.0).step(0.1);

//     let mut chart = ChartBuilder::on(&area)
//         .caption(format!("3D Plot Test"), ("sans", 20))
//         .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;

//     chart.with_projection(|mut pb| {
//         pb.yaw = 0.5;
//         pb.scale = 0.9;
//         pb.into_matrix()
//     });

//     chart
//         .configure_axes()
//         .light_grid_style(BLACK.mix(0.15))
//         .max_light_lines(3)
//         .draw()?;

//     chart
//         .draw_series(
//             SurfaceSeries::xoz(
//                 (-30..30).map(|f| f as f64 / 10.0),
//                 (-30..30).map(|f| f as f64 / 10.0),
//                 |x, z| (x * x + z * z).cos(),
//             )
//             .style(BLUE.mix(0.2).filled()),
//         )?
//         .label("Surface")
//         .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

//     chart
//         .draw_series(LineSeries::new(
//             (-100..100)
//                 .map(|y| y as f64 / 40.0)
//                 .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
//             &BLACK,
//         ))?
//         .label("Line")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

//     chart
//         .configure_series_labels()
//         .border_style(&BLACK)
//         .draw()?;

//     // To avoid the IO failure being ignored silently, we manually call the present function
//     area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
//     println!("Result has been saved to {}", OUT_FILE_NAME);
//     Ok(())
// }
