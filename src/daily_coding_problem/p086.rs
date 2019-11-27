use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a string of parentheses, write a function to
compute the minimum number of parentheses to be removed to make the string valid
(i.e. each open parenthesis is eventually closed).

For example, given the string "()())()", you should return 1. Given the string
")(", you should return 2, since we must remove all of them."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 86"
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
