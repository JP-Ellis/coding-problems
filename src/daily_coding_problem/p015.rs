use crate::Problem;
use std::ops;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 15

Given a stream of elements too large to store in memory, pick a random element
from the stream with uniform probability."#;

struct InfinitCounter<T> {
    val: T,
    step: T,
}

impl<T: num::Zero + num::One> Default for InfinitCounter<T> {
    fn default() -> Self {
        InfinitCounter {
            val: T::zero(),
            step: T::one(),
        }
    }
}

impl<T: num::One + ops::Add<Output = T> + Copy> Iterator for InfinitCounter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.val = self.val + self.step;
        Some(self.val)
    }
}

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
