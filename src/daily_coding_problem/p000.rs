use crate::Problem;

pub struct P;

const STATEMENT: &str = r#""#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        unimplemented!()
    }

    fn bench(&self) {
        unimplemented!()
    }
}
