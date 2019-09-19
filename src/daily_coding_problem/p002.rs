use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str =
    r#"Given an array of integers, return a new array such that each element at index i
of the new array is the product of all the numbers in the original array except
the one at i.

For example, if our input was `[1, 2, 3, 4, 5]`, the expected output would be
`[120, 60, 40, 30, 24]`. If our input was `[3, 2, 1]`, the expected output would
be `[2, 3, 6]`.

Follow-up: what if you can't use division?"#;

fn products_except(list: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(list.len());
    for i in 0..list.len() {
        result.push(
            list[0..i].iter().product::<i64>() * list[i + 1..list.len()].iter().product::<i64>(),
        )
    }
    result
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 2"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        for list in &[&[1, 2, 3, 4, 5][..], &[-5, 0, 2], &[2, 1, 2]] {
            writeln!(out, "{:?} => {:?}", list, products_except(list))?;
        }

        if products_except(&[1, 2, 3, 4, 5]) == vec![120, 60, 40, 30, 24]
            && products_except(&[3, 2, 1]) == vec![2, 3, 6]
        {
            Ok(())
        } else {
            Err(format!(
                "Did not get expected result.\n  {:?} != {:?}\n  {:?} != {:?}",
                products_except(&[1, 2, 3, 4, 5]),
                vec![120, 60, 40, 30, 24],
                products_except(&[3, 2, 1]),
                vec![2, 3, 6]
            ))?
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;

    macro_rules! bench {
        ($f:ident, $n:expr) => {
            #[bench]
            fn $f(b: &mut Bencher) {
                let v: Vec<_> = (0..2i64.pow($n)).collect();

                b.iter(|| test::black_box(super::products_except(&v)));
            }
        };
    }

    bench!(n_01, 01);
    bench!(n_02, 02);
    bench!(n_03, 03);
    bench!(n_04, 04);
    bench!(n_05, 05);
    bench!(n_06, 06);
    bench!(n_07, 07);
    bench!(n_08, 08);
    bench!(n_09, 09);
    bench!(n_10, 10);
}
