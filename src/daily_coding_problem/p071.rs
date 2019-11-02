use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Using a function `rand7()` that returns an integer
from 1 to 7 (inclusive) with uniform probability, implement a function `rand5()`
that returns an integer from 1 to 5 (inclusive)."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 71"
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
