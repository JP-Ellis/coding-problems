use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement an LRU (Least Recently Used) cache. It
should be able to be initialized with a cache size n, and contain the following
methods:

- `set(key, value)`: sets key to value. If there are already n items in the
  cache and we are adding a new item, then it should also remove the least
  recently used item.
- `get(key)`: gets the value at key. If no such key exists, return null.

Each operation should run in O(1) time."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 51"
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
