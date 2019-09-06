use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Given a list of numbers and a number k, return
whether any two numbers from the list add up to k.

For example, given `[10, 15, 3, 7]` and `k` of `17`, return true since `10 + 7` is `17`.

---

Bonus: Can you do this in one pass?
"#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn bench(&self) {
        unimplemented!()
    }
}
