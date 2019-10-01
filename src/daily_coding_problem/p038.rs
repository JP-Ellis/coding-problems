use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"You have an N by N board. Write a function that,
given N, returns the number of possible arrangements of the board where N queens
can be placed on the board without threatening each other, i.e. no two queens
share the same row, column, or diagonal."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 38"
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
