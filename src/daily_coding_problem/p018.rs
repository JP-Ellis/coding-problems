use crate::{Error, Problem};
use std::{collections::VecDeque, io::prelude::*};

pub struct P;

const STATEMENT: &str = r#"Given an array of integers and a number k, where 1 <=
k <= length of the array, compute the maximum values of each subarray of length
k.

For example, given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7,
8, 8], since:

    10 = max(10, 5, 2)
    7 = max(5, 2, 7)
    8 = max(2, 7, 8)
    8 = max(7, 8, 7)

Do this in O(n) time and O(k) space. You can modify the input array in-place and
you do not need to store the results. You can simply print them out as you
compute them."#;

fn max_subsets(list: &[i64], k: usize) -> Vec<i64> {
    assert!(k != 0, "k cannot be 0.");
    // If there is only one sublist of length `k` or less, the resulting vector
    // has only a single result.
    if list.len() <= k {
        if let Some(&max) = list.iter().max() {
            return vec![max];
        } else {
            return vec![];
        }
    }

    let mut result = Vec::new();
    let mut subset: VecDeque<_> = list[..k].iter().cloned().collect();
    result.push(*subset.iter().max().unwrap());

    for &i in &list[k..] {
        subset.push_back(i);
        subset.pop_front();
        result.push(*subset.iter().max().unwrap());
    }

    result
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 18"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for list in &[&[10, 5, 2, 7, 8, 7][..], &[0, 1, 2, 3]] {
            writeln!(out, "{:?}", list).unwrap();
            for k in 1..7 {
                writeln!(out, "{} -> {:?}", k, max_subsets(list, k)).unwrap();
            }
        }

        let result = max_subsets(&[10, 5, 2, 7, 8, 7], 3);
        let expected = vec![10, 7, 8, 8];
        if result != expected {
            Err(format!("Expected {:?} but got {:?}.", expected, result))?
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
