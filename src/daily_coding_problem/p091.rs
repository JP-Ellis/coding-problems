use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"What does the below code snippet print out? How can
we fix the anonymous functions to behave as we'd expect?

```
functions = []
for i in range(10):
    functions.append(lambda : i)

for f in functions:
    print(f())
```
"#;

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem"
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
