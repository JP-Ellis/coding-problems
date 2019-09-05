use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Given a list of numbers and a number k, return
whether any two numbers from the list add up to k.

For example, given `[10, 15, 3, 7]` and `k` of `17`, return true since `10 + 7` is `17`.

---

Bonus: Can you do this in one pass?
"#;

// /// Given a list of elements `l`, return the index of the two elements whose sum
// /// is equal to `k`, or `None` if nothing could be found.
// fn find_sum(l: &[i64], k: i64) -> Option<(usize, usize)> {
//     unimplemented!()
// }

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        unimplemented!()
        // let l = [10, 15, 3, 7];
        // log::info!("Input list: {}", l);
        // if let Some((ai, bi)) = find_sum(&l, 17) {
        //     log::info!("{} + {} = {}", l[ai], b[ai], k)
        // }
    }

    fn bench(&self) {
        unimplemented!()
    }
}
