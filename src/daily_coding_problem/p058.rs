use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"An sorted array of integers was rotated an unknown
number of times.

Given such an array, find the index of the element in the array in faster than
linear time. If the element doesn't exist in the array, return null.

For example, given the array [13, 18, 25, 2, 8, 10] and the element 8, return 4
(the index of 8 in the array).

You can assume all the integers in the array are unique."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 58"
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
