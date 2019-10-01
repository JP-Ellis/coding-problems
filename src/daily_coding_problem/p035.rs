use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an array of strictly the characters 'R', 'G',
and 'B', segregate the values of the array so that all the Rs come first, the Gs
come second, and the Bs come last. You can only swap elements of the array.

Do this in linear time and in-place.

For example, given the array ['G', 'B', 'R', 'R', 'B', 'R', 'G'], it should
become ['R', 'R', 'R', 'G', 'G', 'B', 'B']."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 35"
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
