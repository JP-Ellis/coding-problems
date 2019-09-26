use crate::{Error, Problem};
use rand::prelude::*;
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a stream of elements too large to store in
memory, pick a random element from the stream with uniform probability."#;

fn sample_stream<T, I>(s: I, n: usize) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    assert!(
        n > 0,
        "The number of samples to return must be greater than 1."
    );
    let mut result = Vec::new();
    let rng = &mut rand::thread_rng();
    let dist = rand::distributions::Uniform::new(0, n);

    for (i, si) in s.into_iter().enumerate() {
        if i < n {
            if i == 0 {
                result.push(si);
            } else {
                result.insert(rng.gen_range(0, result.len()), si);
            }
        } else if rng.gen::<f64>() < (n as f64) / (i as f64) {
            result[dist.sample(rng)] = si;
        }
    }

    result
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 15"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for &n in &[1, 2, 5, 10] {
            writeln!(
                out,
                "Random {:>2}-sample for 0..10   : {:?}",
                n,
                sample_stream(0..10, n)
            )
            .unwrap();
            writeln!(
                out,
                "Random {:>2}-sample for 0..100  : {:?}",
                n,
                sample_stream(0..100, n)
            )
            .unwrap();
            writeln!(
                out,
                "Random {:>2}-sample for 0..1000 : {:?}",
                n,
                sample_stream(0..1000, n)
            )
            .unwrap();
        }

        let mut mean = 0.0;
        for _ in 0..1_000 {
            let sum: usize = sample_stream(0..1_000, 10).iter().sum();
            mean += sum as f64 / 1e3;
        }

        let expected = 5e3;
        let std_dev = 30.0;
        if expected - 2.0 * std_dev < mean && mean < expected + 2.0 * std_dev {
            Ok(())
        } else {
            Err(format!(
                "[random] Expected mean: {:.3e}.  Got {:.3e}.",
                expected, mean
            ))?
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
