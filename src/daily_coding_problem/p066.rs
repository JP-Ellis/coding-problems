use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Assume you have access to a function toss_biased()
which returns 0 or 1 with a probability that's not 50-50 (but also not 0-100 or
100-0). You do not know the bias of the coin.

Write a function to simulate an unbiased coin toss."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 66"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        Err(())?
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
