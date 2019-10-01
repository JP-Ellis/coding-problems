use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"The power set of a set is the set of all its
subsets. Write a function that, given a set, generates its power set.

For example, given the set `{1, 2, 3}`, it should return `{{}, {1}, {2}, {3},
{1, 2}, {1, 3}, {2, 3}, {1, 2, 3}}`.

You may also use a list or array to represent a set."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 37"
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
