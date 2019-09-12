use crate::Problem;
use std::collections::HashSet;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 1

Given a list of numbers and a number k, return whether any two numbers from the
list add up to k.

For example, given `[10, 15, 3, 7]` and `k` of `17`, return true since `10 + 7` is `17`.

---

Bonus: Can you do this in one pass?
"#;

fn naive_contains_two_sum(n: i64, list: &[i64]) -> Option<(i64, i64)> {
    // Naive implementation that looks at all pairs
    for (i1, &l1) in list.iter().enumerate() {
        for (i2, &l2) in list.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            if l1 + l2 == n {
                return Some((l1, l2));
            }
        }
    }
    None
}

fn set_contains_two_sum(n: i64, list: &[i64]) -> Option<(i64, i64)> {
    let mut set: HashSet<i64> = HashSet::with_capacity(list.len());
    for &i in list {
        let tmp = n - i;
        if set.contains(&tmp) {
            return Some((tmp, i));
        }
        set.insert(i);
    }
    None
}

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        for list in &[&[1, 5, 3, 4, 2][..], &[2, 6, 0], &[4, -3, 2], &[1, 1, 1]] {
            println!("List: {:?}", list);
            for n in 0..=10 {
                println!(
                    "Contains 2-sum to {}: {:?}",
                    n,
                    set_contains_two_sum(n, list)
                );
            }
        }

        if set_contains_two_sum(17, &[10, 15, 3, 7]) == naive_contains_two_sum(17, &[10, 15, 3, 7])
        {
            Ok(())
        } else {
            Err("Did not find two-sum".to_string())
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;

    macro_rules! bench {
        ($f:ident, $n:expr, $sup:expr) => {
            #[bench]
            fn $f(b: &mut Bencher) {
                let list: Vec<i64> = (0..$n).collect();

                b.iter(|| {
                    for sum in 1..(2 * $n - 3) {
                        assert!($sup(sum, &list).is_some());
                    }
                })
            }
        };
    }

    bench!(naive_0004, 5, super::naive_contains_two_sum);
    bench!(naive_0016, 10, super::naive_contains_two_sum);
    bench!(naive_0064, 50, super::naive_contains_two_sum);
    bench!(naive_0256, 100, super::naive_contains_two_sum);
    bench!(naive_1024, 500, super::naive_contains_two_sum);

    bench!(set_0004, 5, super::set_contains_two_sum);
    bench!(set_0016, 10, super::set_contains_two_sum);
    bench!(set_0064, 50, super::set_contains_two_sum);
    bench!(set_0256, 100, super::set_contains_two_sum);
    bench!(set_1024, 1000, super::set_contains_two_sum);
}
