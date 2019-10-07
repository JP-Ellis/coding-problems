use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement a stack that has the following methods:

- push(val), which pushes an element onto the stack
- pop(), which pops off and returns the topmost element of the stack. If there
  are no elements in the stack, then it should throw an error or return null.
- max(), which returns the maximum value in the stack currently. If there are no
  elements in the stack, then it should throw an error or return null.

Each method should run in constant time."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 43"
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
