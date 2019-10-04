use crate::{Error, Problem};
use std::{cmp::Ordering, io::prelude::*};

pub struct P;

const STATEMENT: &str = r#"Given a list of integers S and a target number k,
write a function that returns a subset of S that adds up to k. If such a subset
cannot be made, then return null.

Integers can appear more than once in the list. You may assume all numbers in
the list are positive.

For example, given S = [12, 1, 61, 5, 9, 2] and k = 24, return [12, 9, 2, 1]
since it sums up to 24."#;

/// Given a list of integers `list`, find all subsets which sum up to `target`.
///
/// If no subset could be found, `None` is returned.
fn sum_subsets(list: &[i64], target: i64) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    for (idx, &i) in list.iter().enumerate() {
        match i.cmp(&target) {
            Ordering::Less => {
                for mut subresult in sum_subsets(&list[idx + 1..], target - i) {
                    subresult.push(i);
                    results.push(subresult);
                }
            }
            Ordering::Equal => (results.push(vec![i])),
            Ordering::Greater => (),
        }
    }

    results
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 42"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for list in &[&[1, 2, 3, 4, 5][..], &[-2, 0, 1, 2, 5]] {
            writeln!(out, "{:?}", list).unwrap();
            for target in 0..=15 {
                writeln!(out, "-> {:>2}: {:?}", target, sum_subsets(list, target)).unwrap();
            }
        }

        let expected = vec![vec![2, 9, 1, 12]];
        let result = sum_subsets(&[12, 1, 61, 5, 9, 2], 24);

        if expected != result {
            Err(format!("Expected {:?} but got {:?}.", expected, result))?
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
