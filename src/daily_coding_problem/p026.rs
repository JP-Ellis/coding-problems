use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a singly linked list and an integer k, remove
the kth last element from the list. k is guaranteed to be smaller than the
length of the list.

The list is very long, so making more than one pass is prohibitively expensive.

Do this in constant space and in one pass."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 26"
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
