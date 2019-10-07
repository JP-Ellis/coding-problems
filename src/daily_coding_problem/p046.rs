use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a string, find the longest palindromic
contiguous substring. If there are more than one with the maximum length, return
any one.

For example, the longest palindromic substring of "aabcdcb" is "bcdcb". The
longest palindromic substring of "bananas" is "anana"."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 46"
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
