use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement integer exponentiation. That is, implement
the `pow(x, y)` function, where `x` and `y` are integers and returns `x^y`.

Do this faster than the naive method of repeated multiplication.

For example, `pow(2, 10)` should return 1024."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 61"
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
