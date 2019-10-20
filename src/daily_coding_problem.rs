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
mod p040;
mod p041;
mod p042;
mod p043;
mod p044;
mod p045;
mod p046;
mod p047;
mod p048;
mod p049;
mod p050;
mod p051;
mod p052;
mod p053;
mod p054;
mod p055;
mod p056;
mod p057;
mod p058;
mod p059;
// mod p060;
// mod p061;
// mod p062;
// mod p063;
// mod p064;
// mod p065;
// mod p066;
// mod p067;
// mod p068;
// mod p069;
// mod p070;
// mod p071;
// mod p072;
// mod p073;
// mod p074;
// mod p075;
// mod p076;
// mod p077;
// mod p078;
// mod p079;
// mod p080;
// mod p081;
// mod p082;
// mod p083;
// mod p084;
// mod p085;
// mod p086;
// mod p087;
// mod p088;
// mod p089;
// mod p090;
// mod p091;
// mod p092;
// mod p093;
// mod p094;
// mod p095;
// mod p096;
// mod p097;
// mod p098;
// mod p099;

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
        40 => Ok(&p040::P),
        41 => Ok(&p041::P),
        42 => Ok(&p042::P),
        43 => Ok(&p043::P),
        44 => Ok(&p044::P),
        45 => Ok(&p045::P),
        46 => Ok(&p046::P),
        47 => Ok(&p047::P),
        48 => Ok(&p048::P),
        49 => Ok(&p049::P),
        50 => Ok(&p050::P),
        51 => Ok(&p051::P),
        52 => Ok(&p052::P),
        53 => Ok(&p053::P),
        54 => Ok(&p054::P),
        55 => Ok(&p055::P),
        56 => Ok(&p056::P),
        57 => Ok(&p057::P),
        58 => Ok(&p058::P),
        59 => Ok(&p059::P),
        // 60 => Ok(&p060::P),
        // 61 => Ok(&p061::P),
        // 62 => Ok(&p062::P),
        // 63 => Ok(&p063::P),
        // 64 => Ok(&p064::P),
        // 65 => Ok(&p065::P),
        // 66 => Ok(&p066::P),
        // 67 => Ok(&p067::P),
        // 68 => Ok(&p068::P),
        // 69 => Ok(&p069::P),
        // 70 => Ok(&p070::P),
        // 71 => Ok(&p071::P),
        // 72 => Ok(&p072::P),
        // 73 => Ok(&p073::P),
        // 74 => Ok(&p074::P),
        // 75 => Ok(&p075::P),
        // 76 => Ok(&p076::P),
        // 77 => Ok(&p077::P),
        // 78 => Ok(&p078::P),
        // 79 => Ok(&p079::P),
        // 80 => Ok(&p080::P),
        // 81 => Ok(&p081::P),
        // 82 => Ok(&p082::P),
        // 83 => Ok(&p083::P),
        // 84 => Ok(&p084::P),
        // 85 => Ok(&p085::P),
        // 86 => Ok(&p086::P),
        // 87 => Ok(&p087::P),
        // 88 => Ok(&p088::P),
        // 89 => Ok(&p089::P),
        // 90 => Ok(&p090::P),
        // 91 => Ok(&p091::P),
        // 92 => Ok(&p092::P),
        // 93 => Ok(&p093::P),
        // 94 => Ok(&p094::P),
        // 95 => Ok(&p095::P),
        // 96 => Ok(&p096::P),
        // 97 => Ok(&p097::P),
        // 98 => Ok(&p098::P),
        // 99 => Ok(&p099::P),
        i => Err(i),
    }
}
