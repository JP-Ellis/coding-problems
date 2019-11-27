use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Determine whether a tree is a valid binary search
tree.

A binary search tree is a tree with two children, left and right, and satisfies
the constraint that the key in the left child must be less than or equal to the
root and the key in the right child must be greater than or equal to the
root."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 89"
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
