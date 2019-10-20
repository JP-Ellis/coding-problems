use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement a file syncing algorithm for two computers
over a low-bandwidth network. What if we know the files in the two computers are
mostly the same?"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 59"
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
