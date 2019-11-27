use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a binary tree of integers, find the maximum
path sum between two nodes. The path must go through at least one node, and does
not need to go through the root."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 94"
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
