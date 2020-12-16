use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        "problem_16"
    }

    fn part1(&self) -> io::Result<i64> {
        todo!()
    }

    fn part2(&self) -> io::Result<i64> {
        todo!()
    }
}

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        Range { min: min, max: max }
    }

    fn contains(&self, value: i64) -> bool {
        self.min <= value && value <= self.max
    }
}

struct DisjunctRange(Vec<Range>);

impl DisjunctRange {
    fn new(ranges: Vec<Range>) -> DisjunctRange {
        DisjunctRange(ranges)
    }

    fn contains(&self, value: i64) -> bool {
        for range in self.0.iter() {
            if range.contains(value) {
                return true;
            }
        }

        false
    }
}

struct ValidationRules(HashMap<String, DisjunctRange>);

impl ValidationRules {
    // essentially the core implementation of problem 16 part 1--are all of the
    // fields of the ticket valid according to at least one range
    fn are_any_valid(&self, t: &Ticket) -> bool {
        for (_, range) in self.0.iter() {
            let mut is_valid = false;
            for field in t.0.iter() {
                if range.contains(*field) {
                    is_valid = true;
                }
            }

            if !is_valid {
                return false;
            }
        }

        true
    }
}

impl FromStr for ValidationRules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Ticket(Vec<i64>);

impl FromStr for Ticket {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec: Vec<i64> = Vec::new();
        for term in s.split(",") {
            vec.push(term.parse::<i64>().map_err(|_| "failed to parse ticket")?);
        }
        Ok(Ticket(vec))
    }
}
