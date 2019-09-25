use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Compute the running median of a sequence of
numbers. That is, given a stream of numbers, print out the median of the list so
far on each new element.

Recall that the median of an even-numbered list is the average of the two middle
numbers.

For example, given the sequence [2, 1, 5, 7, 2, 0, 5], your algorithm should
print out:

```
2
1.5
2
3.5
2
2
2
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 33"
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
