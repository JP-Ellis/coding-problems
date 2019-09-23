use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Run-length encoding is a fast and simple method of
encoding strings. The basic idea is to represent repeated successive characters
as a single count and character. For example, the string "AAAABBBCCDAA" would be
encoded as "4A3B2C1D2A".

Implement run-length encoding and decoding. You can assume the string to be
encoded have no digits and consists solely of alphabetic characters. You can
assume the string to be decoded is valid."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 29"
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
