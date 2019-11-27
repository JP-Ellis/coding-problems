use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a number represented by a list of digits, find
the next greater permutation of a number, in terms of lexicographic ordering. If
there is not greater permutation possible, return the permutation with the
lowest value/ordering.

For example, the list `[1,2,3]` should return `[1,3,2]`. The list `[1,3,2]`
should return `[2,1,3]`. The list `[3,2,1]` should return `[1,2,3]`.

Can you perform the operation without allocating extra memory (disregarding the
input memory)?"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 95"
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
