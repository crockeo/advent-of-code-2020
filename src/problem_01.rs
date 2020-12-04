use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::iter::FromIterator;
use std::path::Path;

pub fn part1() -> io::Result<u32> {
    part_impl(2)
}

pub fn part2() -> io::Result<u32> {
    part_impl(3)
}

fn part_impl(term_count: usize) -> io::Result<u32> {
    let terms = load_terms()?;
    let values = find_summing_terms(&terms, term_count, 2020);

    let mut i = 1;
    for value in values.iter() {
        i *= value
    }
    Ok(i)
}

fn load_terms() -> Result<Vec<u32>> {
    let file = File::open(Path::new(".").join("data").join("01.txt").as_path())?;

    let mut terms = Vec::new();
    for line in BufReader::new(file).lines() {
        let real_line = line?;
        match real_line.parse::<u32>() {
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            Ok(term) => terms.push(term),
        }
    }

    Ok(terms)
}

fn find_summing_terms(terms: &Vec<u32>, term_count: usize, target: u32) -> Vec<u32> {
    // NOTE: this has really bad worst case complexity, but works in practice because it
    // continuously filters out all of the terms that are too big to be feasible.
    //
    // i'm here to futz around with rust, not make good algorithms, come on!
    if term_count == 1 {
        for term in terms.iter() {
            if *term == target {
                return vec![*term];
            }
        }
        return vec![];
    }

    let mut reduced_terms = HashMap::new();
    for term in terms.iter().copied() {
        if term > target {
            continue;
        }
        reduced_terms.insert(term, target - term);
    }

    for (value, reduced) in reduced_terms.iter() {
        let mut summing_terms = find_summing_terms(terms, term_count - 1, *reduced);
        if summing_terms.len() > 0 {
            summing_terms.push(*value);
            return summing_terms;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TERMS: &[u32] = &[1721, 979, 366, 299, 675, 1456];
    static TARGET: u32 = 2020;

    #[test]
    fn test_two_terms() {
        let terms = Vec::from_iter(TERMS.iter().copied());
        let output = find_summing_terms(&terms, 2, TARGET);
        let product: u32 = output.iter().product();

        assert_eq!(output.len(), 2);
        assert_eq!(product, 514579);
    }

    #[test]
    fn test_three_terms() {
        let terms = Vec::from_iter(TERMS.iter().copied());
        let output = find_summing_terms(&terms, 3, TARGET);
        let product: u32 = output.iter().product();

        assert_eq!(output.len(), 3);
        assert_eq!(product, 241861950);
    }
}
