use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given k sorted singly linked lists, write a function
to merge all the lists into one sorted singly linked list."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 78"
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
