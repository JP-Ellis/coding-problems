use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a string s and an integer k, break up the
string into multiple lines such that each line has a length of k or less. You
must break it up so that words don't break across lines. Each line has to have
the maximum possible amount of words. If there's no way to break the text up,
then return null.

You can assume that there are no spaces at the ends of the string and that there
is exactly one space between each word.

For example, given the string "the quick brown fox jumps over the lazy dog" and
k = 10, you should return: ["the quick", "brown fox", "jumps over", "the lazy",
"dog"]. No string in the list has a length of more than 10."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 57"
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
