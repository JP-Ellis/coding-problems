use crate::{Error, Problem};
use std::collections::HashMap;
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an integer k and a string s, find the length of
the longest substring that contains at most k distinct characters.

For example, given s = "abcba" and k = 2, the longest substring with k distinct
characters is "bcb"."#;

/// Fund the longest substring of `s` containing at most `k` distinct
/// characters.
///
/// If the string could contain multiple such substrings, the first is returned.
fn longest_substring(s: &str, k: usize) -> &str {
    let mut best = (0, 0);
    let mut current = (0, 0);
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        current.1 += 1;

        *counts.entry(c).or_default() += 1;
        if counts.len() > k {
            let c0 = s[current.0..current.1].chars().nth(0).unwrap();
            let n0 = counts.get_mut(&c0).unwrap();
            *n0 -= 1;
            if *n0 == 0 {
                counts.remove(&c0);
            }
            current.0 += 1;
        }

        if current.1 - current.0 > best.1 - best.0 {
            best = current;
        }
    }

    &s[best.0..best.1]
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 13"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for s in &["abcdef", "aabbcc", "abcba", "aabbcczccba"] {
            writeln!(out, "{}", s).unwrap();
            for k in 0..5 {
                writeln!(out, "{} -> {}", k, longest_substring(s, k)).unwrap();
            }
        }

        if longest_substring("abcba", 2) != "bcb" {
            Err(format!(
                "Expected '{}' but got '{}'.",
                "bcb",
                longest_substring("abcba", 2)
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
