use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement a URL shortener with the following methods:

- `shorten(url)`, which shortens the url into a six-character alphanumeric
  string, such as `zLg6wl`.
- `restore(short)`, which expands the shortened string into the original url. If
  no such shortened string exists, return null.

Hint: What if we enter the same URL twice?"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 55"
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
