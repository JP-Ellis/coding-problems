use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an undirected graph represented as an adjacency
matrix and an integer k, write a function to determine whether each vertex in
the graph can be colored such that no two adjacent vertices share the same color
using at most k colors."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 56"
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
