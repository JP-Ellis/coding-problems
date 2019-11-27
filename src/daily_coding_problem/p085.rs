use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given three 32-bit integers x, y, and b, return x if
b is 1 and y if b is 0, using only mathematical or bit operations. You can
assume b can only be 1 or 0."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 85"
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
