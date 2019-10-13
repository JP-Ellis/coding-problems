use crate::{Error, Problem};
use rand::prelude::*;
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a function that generates perfectly random
numbers between 1 and k (inclusive), where k is an input, write a function that
shuffles a deck of cards represented as an array using only swaps.

It should run in O(N) time.

Hint: Make sure each one of the 52! permutations of the deck is equally
likely."#;

fn shuffle<T>(l: &mut [T]) {
    let n = l.len();
    let mut rng = thread_rng();

    for i in (1..n).rev() {
        let idx = rng.gen_range(0, i);
        l.swap(i, idx);
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 51"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let mut sums: Vec<usize> = (0..52).map(|_| 0).collect();
        let deck: Vec<_> = (0..52).collect();
        let shuffles = 10_000;
        for _ in 0..shuffles {
            let mut new = deck.clone();
            shuffle(&mut new);
            for (i, &card) in new.iter().enumerate() {
                sums[i] += card;
            }
        }

        let mean = 255000;
        let stdev = 3002;
        for sum in sums {
            if !(mean - 3 * stdev < sum && sum < mean + 3 * stdev) {
                Err(format!(
                    "Result {} is not within 3σ of {} (3σ range: {} - {})",
                    sum,
                    mean,
                    mean - 3 * stdev,
                    mean + 3 * stdev
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
