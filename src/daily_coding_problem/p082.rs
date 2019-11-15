use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Using a read7() method that returns 7 characters from
a file, implement readN(n) which reads n characters.

For example, given a file with the content “Hello world”, three read7() returns
“Hello w”, “orld” and then “”."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 82"
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
