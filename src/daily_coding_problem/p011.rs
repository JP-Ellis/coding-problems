use crate::{Error, Problem};
use std::collections::HashMap;
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Implement an autocomplete system. That is, given a
query string `s` and a set of all possible query strings, return all strings in
the set that have `s` as a prefix.

For example, given the query string de and the set of strings `[dog, deer, deal]`,
return `[deer, deal]`.

Hint: Try preprocessing the dictionary into a more efficient data structure to
speed up queries."#;

#[derive(Default, Debug)]
struct Dictionary {
    ends: Vec<char>,
    sub_dicts: HashMap<char, Dictionary>,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary::default()
    }

    fn insert(&mut self, word: &str) {
        match word.len() {
            0 => (),
            1 => self.ends.push(word.chars().nth(0).unwrap()),
            _ => {
                let entry = self
                    .sub_dicts
                    .entry(word.chars().nth(0).unwrap())
                    .or_default();
                entry.insert(&word[1..]);
            }
        }
    }

    /// Return the completions with the specified start.
    ///
    /// If the start is empty, all completions are returned.
    fn completions(&self, start: &str) -> Vec<String> {
        if start.is_empty() {
            self.all_entries()
        } else {
            let mut completions = Vec::new();

            let c = start.chars().nth(0).unwrap();

            // Add any words that end with the letter if that is the last one
            if start.len() == 1 && self.ends.contains(&c) {
                completions.push(format!("{}", c));
            }

            // Get the next sub-dictionary and add its completions.
            if let Some(dict) = self.sub_dicts.get(&c) {
                completions.extend(
                    dict.completions(&start[1..])
                        .iter()
                        .map(|completion| format!("{}{}", c, completion)),
                );
            }

            completions
        }
    }

    /// Return all dictionary entries.
    fn all_entries(&self) -> Vec<String> {
        let mut completions = Vec::new();

        //
        completions.extend(self.ends.iter().map(|c| format!("{}", c)));

        for (c, d) in &self.sub_dicts {
            completions.extend(
                d.all_entries()
                    .iter()
                    .map(|completion| format!("{}{}", c, completion)),
            )
        }

        completions
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 11"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        let mut dict = Dictionary::new();
        dict.insert("hello");
        dict.insert("hell");
        dict.insert("help");
        dict.insert("other");

        for start in &["", "h", "he", "hel", "hell", "hello"] {
            writeln!(out, "{:<5} => {:?}", start, dict.completions(start))?;
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
