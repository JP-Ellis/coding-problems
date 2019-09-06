use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"A unival tree (which stands for "universal value") is
a tree where all nodes under it have the same value.

Given the root to a binary tree, count the number of unival subtrees.

For example, the following tree has 5 unival subtrees:

```
   0
  / \
 1   0
    / \
   1   0
  / \
 1   1
```
"#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn bench(&self) {
        unimplemented!()
    }
}
