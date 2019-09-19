use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an array of integers and a number k, where 1 <=
k <= length of the array, compute the maximum values of each subarray of length
k.

For example, given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7,
8, 8], since:

    10 = max(10, 5, 2)
    7 = max(5, 2, 7)
    8 = max(2, 7, 8)
    8 = max(7, 8, 7)

Do this in O(n) time and O(k) space. You can modify the input array in-place and
you do not need to store the results. You can simply print them out as you
compute them."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 18"
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
