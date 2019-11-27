use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"We're given a hashmap associating each courseId key
with a list of `courseIds` values, which represents that the prerequisites of
`courseId` are `courseIds`. Return a sorted ordering of courses such that we can
finish all courses.

Return null if there is no such ordering.

For example, given

```
{
 'CSC300': ['CSC100', 'CSC200'],
 'CSC200': ['CSC100'],
 'CSC100': []
}
```

should return `['CSC100', 'CSC200', 'CSCS300']`."#;

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
