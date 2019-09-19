use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"A builder is looking to build a row of N houses that
can be of K different colors. He has a goal of minimizing cost while ensuring
that no two neighboring houses are of the same color.

Given an N by K matrix where the nth row and kth column represents the cost to
build the nth house with kth color, return the minimum cost which achieves this
goal."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 19"
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
