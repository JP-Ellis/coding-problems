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
mod p017;
mod p018;
mod p019;
mod p020;
mod p021;
mod p022;
mod p023;

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
        17 => Ok(Box::new(p017::P)),
        18 => Ok(Box::new(p018::P)),
        19 => Ok(Box::new(p019::P)),
        20 => Ok(Box::new(p020::P)),
        21 => Ok(Box::new(p021::P)),
        22 => Ok(Box::new(p022::P)),
        23 => Ok(Box::new(p023::P)),
        // 24 => Ok(Box::new(p024::P)),
        // 25 => Ok(Box::new(p025::P)),
        // 26 => Ok(Box::new(p026::P)),
        // 27 => Ok(Box::new(p027::P)),
        // 28 => Ok(Box::new(p028::P)),
        // 29 => Ok(Box::new(p029::P)),
        i => Err(format!(
            "Problem '{}' is not knowing within Daily Coding Problems.",
            i
        )),
    }
}
