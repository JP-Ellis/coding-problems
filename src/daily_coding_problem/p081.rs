use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a mapping of digits to letters (as in a phone
number), and a digit string, return all possible letters the number could
represent. You can assume each valid number in the mapping is a single digit.

For example if
```
{“2”: [“a”, “b”, “c”], 3: [“d”, “e”, “f”], …}
```
then “23” should return
```
[“ad”, “ae”, “af”, “bd”, “be”, “bf”, “cd”, “ce”, “cf"]
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 81"
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
