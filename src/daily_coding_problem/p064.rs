use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"A knight's tour is a sequence of moves by a knight on
a chessboard such that all squares are visited once.

Given N, write a function to return the number of knight's tours on an N by N
chessboard."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 64"
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
