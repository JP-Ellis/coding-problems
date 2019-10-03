use crate::{Error, Problem};
use std::io::prelude::*;

pub struct P;

const STATEMENT: &str = r#"Given an unordered list of flights taken by someone,
each represented as (origin, destination) pairs, and a starting airport, compute
the person's itinerary. If no such itinerary exists, return null. If there are
multiple possible itineraries, return the lexicographically smallest one. All
flights must be used in the itinerary.

For example, given the list of flights [('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL',
'YYZ'), ('HKO', 'ORD')] and starting airport 'YUL', you should return the list
['YUL', 'YYZ', 'SFO', 'HKO', 'ORD'].

Given the list of flights [('SFO', 'COM'), ('COM', 'YYZ')] and starting airport
'COM', you should return null.

Given the list of flights [('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')] and
starting airport 'A', you should return the list ['A', 'B', 'C', 'A', 'C'] even
though ['A', 'C', 'A', 'B', 'C'] is also a valid itinerary. However, the first
one is lexicographically smaller."#;

fn longest_chain<T: Eq + Clone>(list: Vec<(T, T)>, itinerary: Vec<T>) -> Vec<Vec<T>> {
    assert!(
        !itinerary.is_empty(),
        "Itinerary cannot be cannot be empty (it must at least contain the
        starting point)."
    );

    if list.is_empty() {
        let mut results = Vec::new();
        results.push(Vec::from(itinerary));
        return results;
    }

    let mut results = Vec::new();
    let last = itinerary.last().unwrap();
    let mut prev_jump = list.first().unwrap();
    for (i, jump) in list.iter().enumerate() {
        // Prevent duplicates from listing too many possible itineraries
        if i > 0 && jump == prev_jump {
            continue;
        }
        prev_jump = jump;
        let (start, end) = jump;

        if last == start {
            let mut sublist = list.clone();
            sublist.remove(i);
            let mut subitinerary = itinerary.clone();
            subitinerary.push(end.clone());
            results.append(&mut longest_chain(sublist, subitinerary));
        }
    }

    results
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 41"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, out: &mut dyn Write) -> Result<(), Error> {
        let mut v = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 1), (1, 0), (0, 1)];
        v.sort_unstable();

        writeln!(out, "{:?}:", v).unwrap();
        for itinerary in &longest_chain(v.clone(), vec![0]) {
            writeln!(out, "-> {:?}", itinerary).unwrap();
        }

        let v = vec![
            ("SFO", "HKO"),
            ("YYZ", "SFO"),
            ("YUL", "YYZ"),
            ("HKO", "ORD"),
        ];
        let r = longest_chain(v, vec!["YUL"]);
        if r.first() != Some(&vec!["YUL", "YYZ", "SFO", "HKO", "ORD"]) {
            Err(format!("Didn't get the expected result: {:?}", r.first()))?
        }

        let mut v = vec![('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')];
        v.sort_unstable();
        let r = longest_chain(v, vec!['A']);
        if r.first() != Some(&vec!['A', 'B', 'C', 'A', 'C']) {
            Err(format!("Didn't get the expected result: {:?}", r.first()))?
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
