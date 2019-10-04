use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a list of integers S and a target number k,
write a function that returns a subset of S that adds up to k. If such a subset
cannot be made, then return null.

Integers can appear more than once in the list. You may assume all numbers in
the list are positive.

For example, given S = [12, 1, 61, 5, 9, 2] and k = 24, return [12, 9, 2, 1]
since it sums up to 24."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 42"
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
