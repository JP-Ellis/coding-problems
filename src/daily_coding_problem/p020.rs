use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 20

Given two singly linked lists that intersect at some point, find the
intersecting node. The lists are non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10, return the
node with value 8.

In this example, assume nodes with the same value are the exact same node
objects.

Do this in O(M + N) time (where M and N are the lengths of the lists) and
constant space."#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        Err("not implemented".to_string())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
