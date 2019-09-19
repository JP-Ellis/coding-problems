use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement a job scheduler which takes in a function f
and an integer n, and calls f after n milliseconds."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 10"
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
