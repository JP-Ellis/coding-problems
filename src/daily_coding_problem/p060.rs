use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a multiset of integers, return whether it can
be partitioned into two subsets whose sums are the same.

For example, given the multiset `{15, 5, 20, 10, 35, 15, 10}`, it would return
true, since we can split it up into `{15, 5, 10, 15, 10}` and `{20, 35}`, which
both add up to `55`.

Given the multiset `{15, 5, 20, 10, 35}`, it would return false, since we can't
split it up into two subsets that add up to the same sum."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 60"
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
