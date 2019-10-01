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
mod p024;
mod p025;
mod p026;
mod p027;
mod p028;
mod p029;
mod p030;
mod p031;
mod p032;
mod p033;
mod p034;
mod p035;
mod p036;
mod p037;
mod p038;
mod p039;

pub fn problem(i: usize) -> Result<&'static dyn Problem, usize> {
    match i {
        1 => Ok(&p001::P),
        2 => Ok(&p002::P),
        3 => Ok(&p003::P),
        4 => Ok(&p004::P),
        5 => Ok(&p005::P),
        6 => Ok(&p006::P),
        7 => Ok(&p007::P),
        8 => Ok(&p008::P),
        9 => Ok(&p009::P),
        10 => Ok(&p010::P),
        11 => Ok(&p011::P),
        12 => Ok(&p012::P),
        13 => Ok(&p013::P),
        14 => Ok(&p014::P),
        15 => Ok(&p015::P),
        16 => Ok(&p016::P),
        17 => Ok(&p017::P),
        18 => Ok(&p018::P),
        19 => Ok(&p019::P),
        20 => Ok(&p020::P),
        21 => Ok(&p021::P),
        22 => Ok(&p022::P),
        23 => Ok(&p023::P),
        24 => Ok(&p024::P),
        25 => Ok(&p025::P),
        26 => Ok(&p026::P),
        27 => Ok(&p027::P),
        28 => Ok(&p028::P),
        29 => Ok(&p029::P),
        30 => Ok(&p030::P),
        31 => Ok(&p031::P),
        32 => Ok(&p032::P),
        33 => Ok(&p033::P),
        34 => Ok(&p034::P),
        35 => Ok(&p035::P),
        36 => Ok(&p036::P),
        37 => Ok(&p037::P),
        38 => Ok(&p038::P),
        39 => Ok(&p039::P),
        i => Err(i),
    }
}
