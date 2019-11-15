use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given the root of a binary tree, return a deepest
node. For example, in the following tree, return d.

```
    a
   / \
  b   c
 /
d
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 80"
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
