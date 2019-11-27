use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a tree, find the largest tree/subtree that is a
BST.

Given a tree, return the size of the largest tree/subtree that is a BST."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem"
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
