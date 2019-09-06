use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Implement a job scheduler which takes in a function f
and an integer n, and calls f after n milliseconds."#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        Err("not implemented".to_string())
    }
}
