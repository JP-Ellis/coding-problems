use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 8

A unival tree (which stands for "universal value") is a tree where all nodes
under it have the same value.

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

struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn leaf(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

impl<T: Eq> Node<T> {
    fn count_universal_subtrees(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(t), None) | (None, Some(t)) => {
                t.count_universal_subtrees() + if t.val == self.val { 1 } else { 0 }
            }
            (Some(l), Some(r)) => {
                l.count_universal_subtrees()
                    + r.count_universal_subtrees()
                    + if l.val == self.val && r.val == self.val {
                        1
                    } else {
                        0
                    }
            }
        }
    }
}

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        let tree0 = Node::leaf(true);
        let tree1 = Node {
            val: true,
            left: Some(Box::new(Node::leaf(true))),
            right: Some(Box::new(Node::leaf(true))),
        };
        let tree2 = Node {
            val: false,
            left: Some(Box::new(Node {
                val: true,
                left: Some(Box::new(Node::leaf(true))),
                right: Some(Box::new(Node::leaf(true))),
            })),
            right: Some(Box::new(Node::leaf(false))),
        };
        let tree3 = Node {
            val: false,
            left: Some(Box::new(Node::leaf(true))),
            right: Some(Box::new(Node {
                val: false,
                left: Some(Box::new(Node {
                    val: true,
                    left: Some(Box::new(Node::leaf(true))),
                    right: Some(Box::new(Node::leaf(true))),
                })),
                right: Some(Box::new(Node::leaf(false))),
            })),
        };

        println!("Tree 0: {}", tree0.count_universal_subtrees());
        println!("Tree 1: {}", tree1.count_universal_subtrees());
        println!("Tree 2: {}", tree2.count_universal_subtrees());
        println!("Tree 3: {}", tree3.count_universal_subtrees());

        if tree3.count_universal_subtrees() == 5 {
            Ok(())
        } else {
            Err("Did not calculated the correct number of subtrees.".to_string())
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
