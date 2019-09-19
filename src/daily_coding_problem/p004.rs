use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an array of integers, find the first missing
positive integer in linear time and constant space. In other words, find the
lowest positive integer that does not exist in the array. The array can contain
duplicates and negative numbers as well.

For example, the input `[3, 4, -1, 1]` should give `2`. The input `[1, 2, 0]`
should give `3`.

You can modify the input array in-place."#;

fn find_missing(list: &mut [i64]) -> i64 {
    // Sort the array and get the index of the 1 (or where the one should be)
    list.sort_unstable();
    let start = match list.binary_search(&1) {
        Ok(i) => i,
        Err(i) => i,
    };

    let mut n = 0;
    for &ni in &list[start..] {
        if ni == n {
            continue;
        } else if ni == n + 1 {
            n += 1;
            continue;
        }
        break;
    }
    n + 1
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 4"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for list in &mut [
            &mut [3, 4, -1, 1][..],
            &mut [1, 2, 0],
            &mut [-1, 0, 1, 2, 3],
            &mut [2, -1, -1],
            &mut [1, 1, 2, 3, 4, 3, 5, 3],
            &mut [],
        ] {
            write!(out, "List: {:?} => ", list)?;
            writeln!(out, "{}", find_missing(list))?;
        }

        if find_missing(&mut [3, 4, -1, 1]) == 2 && find_missing(&mut [1, 2, 0]) == 3 {
            Ok(())
        } else {
            Err("Incorrectly found the correct missing integer.")?
        }
    }
}
