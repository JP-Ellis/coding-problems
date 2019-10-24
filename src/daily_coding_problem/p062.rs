use crate::{Error, Problem};
use std::{io::prelude::*, iter};

pub struct P;

const STATEMENT: &str = r#"There is an N by M matrix of zeroes. Given N and M,
write a function to count the number of ways of starting at the top-left corner
and getting to the bottom-right corner. You can only move right or down.

For example, given a 2 by 2 matrix, you should return 2, since there are two
ways to get to the bottom-right:

- Right, then down
- Down, then right

Given a 5 by 5 matrix, there are 70 ways to get to the bottom-right."#;

/// Compute the number of paths going from the `[1, ..., 1]` to `dims`, where
/// each move is in just one direction at a time, always increasing towards
/// `dims`.
fn paths(dims: &[usize]) -> usize {
    let r = dims
        .iter()
        .enumerate()
        .filter(|(_, &n)| n > 1)
        .map(|(i, _)| {
            let mut dims = Vec::from(dims);
            dims[i] -= 1;
            paths(&dims)
        })
        .sum();
    if r == 0 {
        1
    } else {
        r
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 62"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        write!(out, "x  ").unwrap();
        for m in 1..=10 {
            write!(out, "{:<8}", m).unwrap();
        }
        writeln!(out, "").unwrap();
        for n in 1..=10 {
            write!(out, "{:<2}|", n).unwrap();
            for m in 1..=10 {
                write!(out, "{:<8}", paths(&[n, m])).unwrap();

                if paths(&[n, m]) != paths(&[m, n]) {
                    Err(format!("The paths functions is not symmetry."))?
                }
            }
            writeln!(out, "").unwrap();
        }

        for n in 1..=6 {
            let dims: Vec<_> = iter::repeat(2).take(n).collect();
            writeln!(out, "2^{} : {}", n, paths(&dims)).unwrap();
        }

        if paths(&[2, 2]) != 2 {
            Err(format!(
                "2x2 should give 2 paths, but got {}",
                paths(&[2, 2])
            ))?
        }
        if paths(&[5, 5]) != 70 {
            Err(format!(
                "5x5 should give 70 paths, but got {}",
                paths(&[5, 5])
            ))?
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
