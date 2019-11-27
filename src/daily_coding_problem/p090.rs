use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an integer `n` and a list of integers `l`,
write a function that randomly generates a number from `0` to `n-1` that isn't
in `l` (uniform)."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 90"
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
