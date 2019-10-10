use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a array of numbers representing the stock
prices of a company in chronological order, write a function that calculates the
maximum profit you could have made from buying and selling that stock once. You
must buy before you can sell it.

For example, given [9, 11, 8, 5, 7, 10], you should return 5, since you could
buy the stock at 5 dollars and sell it at 10 dollars."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 47"
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
