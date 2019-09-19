use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an array of time intervals (start, end) for
classroom lectures (possibly overlapping), find the minimum number of rooms
required.

For example, given [(30, 75), (0, 50), (60, 150)], you should return 2."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 21"
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
