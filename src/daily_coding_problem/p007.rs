use crate::Problem;
use std::collections::HashMap;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 7

Given the mapping `a = 1`, `b = 2`, ... `z = 26`, and an encoded message, count
the number of ways it can be decoded.

For example, the message `'111'` would give `3`, since it could be decoded as `'aaa'`,
`'ka'`, and `'ak'`.

You can assume that the messages are decodable. For example, `'001'` is not
allowed."#;

/// Given an encoding specified by the HashMap (where the keys and values are
/// the matching encoded and decoded values), return all possible decodings of
/// the input string.
fn decode(map: &HashMap<String, String>, s: &str) -> Vec<String> {
    let mut results = Vec::new();
    if s.is_empty() {
        return results;
    }

    for (k, v) in map {
        if s.starts_with(k) {
            if s == k {
                results.push(v.to_string());
            } else {
                for end in decode(map, &s[k.len()..]) {
                    results.push(format!("{}{}", v, end))
                }
            }
        }
    }

    results
}

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        let map = [
            ("1", "a"),
            ("2", "b"),
            ("3", "c"),
            ("4", "d"),
            ("5", "e"),
            ("6", "f"),
            ("7", "g"),
            ("8", "h"),
            ("9", "i"),
            ("10", "j"),
            ("11", "k"),
            ("12", "l"),
            ("13", "m"),
            ("14", "n"),
            ("15", "o"),
            ("16", "p"),
            ("17", "q"),
            ("18", "r"),
            ("19", "s"),
            ("20", "t"),
            ("21", "u"),
            ("22", "v"),
            ("23", "w"),
            ("24", "x"),
            ("25", "y"),
            ("26", "z"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        for message in &["1", "111", "001", "123456789", "101112"] {
            println!("{} => {:?}", message, decode(&map, message));
        }

        if decode(&map, "111").len() == 3 {
            Ok(())
        } else {
            Err("Decoding '111' gave the wrong number of results.".to_string())
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
