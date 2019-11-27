use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement division of two positive integers without
using the division, multiplication, or modulus operators. Return the quotient as
an integer, ignoring the remainder."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 88"
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
