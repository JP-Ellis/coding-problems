use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given a number in the form of a list of digits,
return all possible permutations.

For example, given `[1, 2, 3]`, return
```
[[1, 2, 3],
 [1, 3, 2],
 [2, 1, 3],
 [2, 3, 1],
 [3, 1, 2],
 [3, 2, 1]]
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 96"
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
