use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Sudoku is a puzzle where you're given a
partially-filled 9 by 9 grid with digits. The objective is to fill the grid with
the constraint that every row, column, and box (3 by 3 subgrid) must contain all
of the digits from 1 to 9.

Implement an efficient sudoku solver."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 54"
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
