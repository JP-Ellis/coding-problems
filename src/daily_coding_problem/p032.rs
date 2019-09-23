use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#" Suppose you are given a table of currency exchange
rates, represented as a 2D array. Determine whether there is a possible
arbitrage: that is, whether there is some sequence of trades you can make,
starting with some amount A of any currency, so that you can end up with some
amount greater than A of that currency.

There are no transaction costs and you can trade fractional quantities."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 32"
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
