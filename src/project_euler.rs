//! Collection of problems from [Project Euler](https://projecteuler.net/).

use crate::Problem;

mod p001;

pub fn problem(i: usize) -> Result<Box<dyn Problem>, String> {
    match i {
        1 => Ok(Box::new(p001::P)),
        i => Err(format!(
            "Problem '{}' is not knowing within Daily Coding Problems.",
            i
        )),
    }
}
