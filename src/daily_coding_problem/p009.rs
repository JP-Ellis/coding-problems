use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a list of integers, write a function that
returns the largest sum of non-adjacent numbers. Numbers can be 0 or negative.

For example, `[2, 4, 6, 2, 5]` should return `13`, since we pick `2`, `6`, and
`5`. `[5, 1, 1, 5]` should return `10`, since we pick `5` and `5`.

Follow-up: Can you do this in O(N) time and constant space?"#;

/// Calculate the largest sum of non-adjacent elements in the list.
///
/// If the list is empty, 0 is returned.  If the list consists of purely
/// non-positive integers, the largest element will be returned.
fn max_sum(l: &[i64]) -> i64 {
    match l.len() {
        0 => 0,
        1 => l[0],
        2 => *l.iter().max().unwrap(),
        3 => *[l[0], l[0] + l[2], l[1], l[2]].iter().max().unwrap(),
        _4_or_more => {
            let s0 = max_sum(&l[2..]);
            let s1 = max_sum(&l[3..]);
            *[l[0], l[0] + s0, s0, l[1], l[1] + s1, s1]
                .iter()
                .max()
                .unwrap()
        }
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 9"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for list in &[
            &[1, 5, 3, 4, 2][..],
            &[2, 6, 0],
            &[4, -3, 2],
            &[-5, -2, -3, -1],
            &[-1, -1, 9],
            &[2, 4, 6, 2, 5],
            &[5, 1, 1, 5],
        ] {
            writeln!(out, "{:?} => {}", list, max_sum(list))?;
        }

        if max_sum(&[2, 4, 6, 2, 5]) == 13 && max_sum(&[5, 1, 1, 5]) == 10 {
            Ok(())
        } else {
            Err("Did not get the desired sum.")?
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
