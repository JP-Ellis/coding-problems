use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"The edit distance between two strings refers to the
minimum number of character insertions, deletions, and substitutions required to
change one string to the other. For example, the edit distance between “kitten”
and “sitting” is three: substitute the “k” for “s”, substitute the “e” for “i”,
and append a “g”.

Given two strings, compute the edit distance between them."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 31"
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
