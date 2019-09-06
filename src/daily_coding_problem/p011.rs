use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Implement an autocomplete system. That is, given a
query string `s` and a set of all possible query strings, return all strings in
the set that have `s` as a prefix.

For example, given the query string de and the set of strings `[dog, deer, deal]`,
return `[deer, deal]`.

Hint: Try preprocessing the dictionary into a more efficient data structure to
speed up queries."#;

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        Err("not implemented".to_string())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
