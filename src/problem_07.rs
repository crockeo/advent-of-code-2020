use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use lazy_static::lazy_static;
use regex::Regex;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        "problem_07"
    }

    fn part1(&self) -> io::Result<i64> {
        let expansion_rules = load_expansion_rules(
            BufReader::new(File::open("./data/07.txt")?)
                .lines()
                .map(|line| line.unwrap()),
        );

        Ok(0)
    }

    fn part2(&self) -> io::Result<i64> {
        Ok(0)
    }
}

type ExpansionRules<'a> = HashMap<&'a str, HashMap<&'a str, i64>>;

fn can_contain_target<'a>(
    bag_type: &'a str,
    target: &'a str,
    rule: &ExpansionRules<'a>,
    acc: &mut HashMap<&'a str, bool>,
) -> bool {
    if *acc.get(bag_type).unwrap_or(&false) {
        return true;
    }

    let sub_bags = match rule.get(bag_type) {
        None => {
            acc.insert(bag_type, false);
            return false;
        }
        Some(sub_bags) => sub_bags,
    };

    if sub_bags.contains_key(target) {
        acc.insert(bag_type, true);
        return true;
    }

    for (sub_bag, _) in sub_bags.iter() {
        if *acc.get(sub_bag).unwrap_or(&false) {
            acc.insert(bag_type, true);
            return true;
        }

        if can_contain_target(sub_bag, target, rule, acc) {
            acc.insert(bag_type, true);
            return true;
        }
    }

    acc.insert(bag_type, false);
    false
}

fn load_expansion_rules<'a, I: Iterator<Item = String>>(i: I) -> Option<ExpansionRules<'a>> {
    todo!()
}

fn load_expansion_rule<'a>(s: &'a str) -> Option<(&'a str, HashMap<&'a str, i64>)> {
    // TODO: perform parsing here, which is not fun
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_expansion_rule() {
        let expansion_rule = load_expansion_rule("plaid magenta bags contain 2 clear lavender bags, 3 clear teal bags, 4 vibrant gold bags.");
        assert_ne!(expansion_rule, None);
        let (root_bag, mut expansion_bags) = expansion_rule.unwrap();

        assert_eq!(root_bag, "plaid magenta");

        assert_eq!(expansion_bags.remove("clear lavender"), Some(2));
        assert_eq!(expansion_bags.remove("clear teal"), Some(3));
        assert_eq!(expansion_bags.remove("vibrant gold"), Some(4));
    }
}
