use crate::{Error, Problem};
use std::{collections::HashMap, io::prelude::*};

pub struct P;

const STATEMENT: &str = r#"Given an array of integers where every integer occurs
three times except for one integer, which only occurs once, find and return the
non-duplicated integer.

For example, given [6, 1, 3, 3, 3, 6, 6], return 1. Given [13, 19, 13, 13],
return 19.

Do this in O(N) time and O(1) space."#;

/// Find the element which is unique within the list, using a HashMap.
///
/// Time: O(n + n / 3)
/// Space: O(n / 3)
fn find_unique_map(list: &[i64]) -> i64 {
    let mut map = HashMap::new();
    for i in list {
        *map.entry(i).or_insert(0) += 1;
    }
    for (&&n, &count) in map.iter() {
        if count == 1 {
            return n;
        }
    }
    unreachable!()
}

/// Find the element which is unique within the list.
///
/// This solution is O(n) time, and O(1) space (although the space can be large)
fn find_unique_bits(list: &[i64]) -> i64 {
    let mut counts = Vec::with_capacity(64);
    for _ in 0..64 {
        counts.push(0)
    }

    for i in list {
        for offset in 0..64 {
            if i & (1 << offset) != 0 {
                counts[offset] += 1;
            }
        }
    }

    let mut result = 0;
    for offset in 0..64 {
        if counts[offset] % 3 == 1 {
            result |= 1 << offset;
        }
    }

    result
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 40"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let result = find_unique_bits(&[6, 1, 3, 3, 3, 6, 6]);
        if result != 1 {
            Err(format!("Expected 1 but got {}.", result))?
        }
        let result = find_unique_map(&[6, 1, 3, 3, 3, 6, 6]);
        if result != 1 {
            Err(format!("Expected 1 but got {}.", result))?
        }

        let result = find_unique_bits(&[13, 19, 13, 13]);
        if result != 19 {
            Err(format!("Expected 1 but got {}.", result))?
        }
        let result = find_unique_map(&[13, 19, 13, 13]);
        if result != 19 {
            Err(format!("Expected 1 but got {}.", result))?
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
