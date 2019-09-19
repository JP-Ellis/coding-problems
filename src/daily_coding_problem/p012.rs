use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"There exists a staircase with N steps, and you can
climb up either 1 or 2 steps at a time. Given N, write a function that returns
the number of unique ways you can climb the staircase. The order of the steps
matters.

For example, if N is 4, then there are 5 unique ways:

 -  1, 1, 1, 1
 -  2, 1, 1
 -  1, 2, 1
 -  1, 1, 2
 -  2, 2

What if, instead of being able to climb 1 or 2 steps at a time, you could climb
any number from a set of positive integers X? For example, if X = {1, 3, 5}, you
could climb 1, 3, or 5 steps at a time."#;

/// Calculate the number of ways the stairs can be climbed.
///
/// Given a stair case of length `n` and the set of jumps allowed `jumps`,
/// return the number of jumps.  If the jumps cannot be computed, return an
/// error.
fn combinations(n: usize, jumps: &[usize]) -> Result<usize, Error> {
    jumps.iter().fold(Ok(0), |s, &jump| {
        if jump == 0 {
            Err("0-sized jumped are not supported.")?
        } else if let Ok(s) = s {
            match n.checked_sub(jump) {
                Some(0) => Ok(s + 1),
                Some(sub_n) => match combinations(sub_n, jumps) {
                    Ok(sub_s) => Ok(s + sub_s),
                    e => e,
                },
                None => Ok(s),
            }
        } else {
            s
        }
    })
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 12"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        // Check various valid combinations
        for jumps in &[&[1, 2][..], &[1, 3, 5], &[2, 5], &[1, 1]] {
            writeln!(out, "Jumps = {:?}.  Combinations:", jumps)?;
            for n in 0..10 {
                writeln!(
                    out,
                    "-> n = {} : {:>3} combinations",
                    n,
                    combinations(n, jumps)?
                )?;
            }
        }

        // Check that 0-sized jumps produce an error.
        let jumps = &[1, 2, 0];
        writeln!(out, "Jumps = {:?} : {:?}", jumps, combinations(5, jumps))?;

        // Check the given test case
        if combinations(4, &[1, 2])? != 5 {
            Err(format!(
                "Expected 5 combinatinos, instead got {}.",
                combinations(4, &[1, 2])?
            ))?
        } else {
            Ok(())
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
