//! Collection of problems from [Daily Coding
//! Problems](https://dailycodingproblem.com).

use crate::Problem;

mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;
mod p011;
mod p012;
mod p013;
mod p014;
mod p015;
mod p016;

pub fn problem(i: usize) -> Result<Box<dyn Problem>, String> {
    match i {
        1 => Ok(Box::new(p001::P)),
        2 => Ok(Box::new(p002::P)),
        3 => Ok(Box::new(p003::P)),
        4 => Ok(Box::new(p004::P)),
        5 => Ok(Box::new(p005::P)),
        6 => Ok(Box::new(p006::P)),
        7 => Ok(Box::new(p007::P)),
        8 => Ok(Box::new(p008::P)),
        9 => Ok(Box::new(p009::P)),
        10 => Ok(Box::new(p010::P)),
        11 => Ok(Box::new(p011::P)),
        12 => Ok(Box::new(p012::P)),
        13 => Ok(Box::new(p013::P)),
        14 => Ok(Box::new(p014::P)),
        15 => Ok(Box::new(p015::P)),
        16 => Ok(Box::new(p016::P)),
        i => Err(format!(
            "Problem '{}' is not knowing within Daily Coding Problems.",
            i
        )),
    }
}
