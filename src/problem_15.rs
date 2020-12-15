use std::collections::HashMap;
use std::io;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        "problem_15"
    }

    fn part1(&self) -> io::Result<i64> {
        Ok(number_game(vec![0, 13, 16, 17, 1, 10, 6], 2020))
    }

    fn part2(&self) -> io::Result<i64> {
        Ok(number_game(vec![0, 13, 16, 17, 1, 10, 6], 30000000))
    }
}

fn number_game(starting_numbers: Vec<i64>, nth: i64) -> i64 {
    let mut round: i64 = 0;

    let mut last_said = HashMap::<i64, i64>::new();
    for starting_number in starting_numbers.into_iter() {
        round += 1;
        last_said.insert(starting_number, round);
    }

    // NOTE: assumes that we never repeat numbers in the input
    let mut last_number_said: i64 = 0;
    let mut current_number: i64 = 0;
    while round < nth {
        round += 1;
        let next_number = if !last_said.contains_key(&current_number) {
            0
        } else {
            round - last_said.get(&current_number).unwrap()
        };
        last_said.insert(current_number, round);
        last_number_said = current_number;
        current_number = next_number;
    }

    last_number_said
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_game() {
        assert_eq!(number_game(vec![0, 3, 6], 2020), 436);
    }
}
