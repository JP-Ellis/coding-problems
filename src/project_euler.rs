//! Collection of problems from [Project Euler](https://projecteuler.net/).

use crate::Problem;

mod p001;

pub fn problem(i: usize) -> Result<&'static dyn Problem, usize> {
    match i {
        1 => Ok(&p001::P),
        i => Err(i),
    }
}
