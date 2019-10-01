use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a string, find the palindrome that can be made
by inserting the fewest number of characters as possible anywhere in the
word. If there is more than one palindrome of minimum length that can be made,
return the lexicographically earliest one (the first one alphabetically).

For example, given the string "race", you should return "ecarace", since we can
add three letters to it (which is the smallest amount to make a
palindrome). There are seven other palindromes that can be made from "race" by
adding three letters, but "ecarace" comes first alphabetically.

As another example, given the string "google", you should return "elgoogle"."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 34"
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
