use crate::Problem;
use rand::prelude::*;
use std::{f64, f64::consts::PI};

pub struct P;

const STATEMENT: &str = r#"The area of a circle is defined as πr². Estimate π to
3 decimal places using a Monte Carlo method.

Hint: The basic equation of a circle is x² + y² = r²."#;

/// Estimate the value of π using Monte Carlo with `n` samples.
fn estimate_pi(n: u64) -> f64 {
    let mut inside = 0u64;
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let (x, y): (f64, f64) = (rng.gen(), rng.gen());
        if x.powi(2) + y.powi(2) < 1.0 {
            inside += 1
        }
    }

    4.0 * inside as f64 / n as f64
}

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        let mut pi_mc = 0.0;
        for n_pow in 1..10 {
            pi_mc = estimate_pi(10u64.pow(n_pow));
            println!(
                "Approximation of π using 10^{} samples: {:<12}  (Δ = {:.2e})",
                n_pow,
                pi_mc,
                (pi_mc - PI).abs()
            );
        }

        if (pi_mc - PI).abs() < 1e-3 {
            Ok(())
        } else {
            Err(format!(
                "Estimate value of π: {}.  Error: {}",
                pi_mc,
                (pi_mc - PI).abs()
            ))
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
