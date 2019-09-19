use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"You run an e-commerce website and want to record the
last N order ids in a log. Implement a data structure to accomplish this, with
the following API:

- `record(order_id)`: adds the order_id to the log
- `get_last(i)`: gets the ith last element from the log. i is guaranteed to be
  smaller than or equal to N.

You should be as efficient with time and space as possible."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 16"
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
