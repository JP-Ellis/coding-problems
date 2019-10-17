use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement a queue using two stacks. Recall that a
queue is a FIFO (first-in, first-out) data structure with the following methods:
- `enqueue`, which inserts an element into the queue, and
- `dequeue`, which removes it."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 53"
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
