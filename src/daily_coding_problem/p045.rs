use crate::{Error, Problem};
use rand::{distributions::Uniform, prelude::*};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Using a function `rand5()` that returns an integer
from 1 to 5 (inclusive) with uniform probability, implement a function `rand7()`
that returns an integer from 1 to 7 (inclusive)."#;

/// Use the random function `f` that is uniform over [0, p) to generate a random
/// number over [0, q).
///
/// # Panics
///
/// `p` must be greater than 1 and `q` cannot be 0.
fn rand_p_to_q<F, R>(f: &F, p: usize, q: usize, rng: &mut R) -> usize
where
    F: Fn(&mut R) -> usize,
    R: Rng + ?Sized,
{
    assert!(p > 1, "p must be greater than 1.");
    assert!(q != 0, "q cannot be 0.");

    // We use integer division rounded down if p > q, and integer division
    // rounded up if q > p.
    match (p / q, (p + q - 1) / p) {
        (1, 1) => f(rng),
        (0, n) if n > 0 => {
            // We are increasing the uniform distribution.  We divide up the
            // larger interval into `n` sub-intervals of size `p`.  Since the
            // last sub-interval may not contain `p` elements, we pad it but
            // discard the result if it is too big and restart.
            let mut r = rand_p_to_q(f, p, n, rng) * p + f(rng);
            while r >= q {
                r = rand_p_to_q(f, p, n, rng) * p + f(rng);
            }
            r
        }
        (n, 1) if n > 0 => {
            // We are decreasing the uniform distribution, with the sub-interval
            // fitting `n` times in the larger one.  We get the result by
            // sampling `n * q` (which is guaranteed to be smaller than or equal
            // to p) and take the result modulo q.
            let max = n * q;
            let mut r = f(rng);
            while r >= max {
                r = f(rng);
            }
            r % q
        }
        (a, b) => panic!(
            "({p} % {q}, {q} % {p}) = ({a}, {b})",
            p = p,
            q = q,
            a = a,
            b = b
        ),
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 45"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let r3 = Uniform::new(0, 3);
        let r29 = Uniform::new(0, 29);
        let rng = &mut thread_rng();

        let mut counts3: Vec<_> = (0..3).map(|_| 0).collect();
        let mut counts29: Vec<_> = (0..29).map(|_| 0).collect();

        let trials = 1000;
        for _ in 0..(3 * 29 * trials) {
            counts3[rand_p_to_q(&|rng| r29.sample(rng), 29, 3, rng)] += 1;
            counts29[rand_p_to_q(&|rng| r3.sample(rng), 3, 29, rng)] += 1;
        }

        // σ * √N * mean(0..3), σ = 1.118
        let sigma = 190;
        let expected = 29 * trials;
        for &count in &counts3 {
            if !(expected - 2 * sigma < count && count < expected + 2 * sigma) {
                Err(format!(
                    "Got a count outside the 2 sigma expectation ({}, expected: {})",
                    count, expected
                ))?
            }
        }

        // σ * √N * mean(0..29), σ = 8.655
        let sigma = 474;
        let expected = 3 * trials;
        for &count in &counts29 {
            if !(expected - 2 * sigma < count && count < expected + 3 * sigma) {
                Err(format!(
                    "Got a count outside the 2 sigma expectation ({}, expected: {})",
                    count, expected
                ))?
            }
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
