use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"You are given an M by N matrix consisting of booleans
that represents a board. Each True boolean represents a wall. Each False boolean
represents a tile you can walk on.

Given this matrix, a start coordinate, and an end coordinate, return the minimum
number of steps required to reach the end coordinate from the start. If there is
no possible path, then return null. You can move up, left, down, and right. You
cannot move through walls. You cannot wrap around the edges of the board.

For example, given the following board:

```
[[f, f, f, f],
 [t, t, f, t],
 [f, f, f, f],
 [f, f, f, f]]
```

and start = `(3, 0)` (bottom left) and end = `(0, 0)` (top left), the minimum
number of steps required to reach the end is 7, since we would need to go
through (1, 2) because there is a wall everywhere else on the second row."#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 23"
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
