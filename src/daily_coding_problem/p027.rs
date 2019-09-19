use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a string of round, curly, and square open and
closing brackets, return whether the brackets are balanced (well-formed).

For example, given the string "([])[]({})", you should return true.

Given the string "([)]" or "((()", you should return false."#;

/// Check whether the string `s` is balanced, given the specified pairs of
/// delimiters.
///
/// The delimiters must be distinct.
fn is_balanced(s: &str, pairs: &[(char, char)]) -> bool {
    let mut next = Vec::new();

    for c in s.chars() {
        // If we're expecting a closing bracket, check for that first and
        // foremost.
        if let Some(&end) = next.last() {
            if c == end {
                next.pop();
                continue;
            }
        }

        for &(start, end) in pairs {
            // Next check if we have an opening bracket
            if c == start {
                next.push(end);
                continue;
            }

            // Otherwise, if we encounter any unexpected closing bracket, the
            // string is invalid.
            if c == end {
                return false;
            }
        }
    }

    next.is_empty()
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 27"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let pairs = vec![('(', ')'), ('[', ']'), ('{', '}')];

        let s = "([])[]({})";
        if !is_balanced(s, &pairs) {
            Err(format!("The string \"{}\" should have been balanced.", s))?
        }

        let s = "([)]";
        if is_balanced(s, &pairs) {
            Err(format!(
                "The string \"{}\" should not have been balanced.",
                s
            ))?
        }

        let s = "((()";
        if is_balanced(s, &pairs) {
            Err(format!(
                "The string \"{}\" should not have been balanced.",
                s
            ))?
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
