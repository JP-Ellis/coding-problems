use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#" Given a matrix of 1s and 0s, return the number of
"islands" in the matrix. A 1 represents land and 0 represents water, so an
island is a group of 1s that are neighboring whose perimeter is surrounded by
water.

For example, this matrix has 4 islands.

```
1 0 0 0 0
0 0 1 1 0
0 1 1 0 0
0 0 0 0 0
1 1 0 0 1
1 1 0 0 1
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 84"
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
