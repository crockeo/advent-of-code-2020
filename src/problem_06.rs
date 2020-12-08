use std::fs::File;
use std::io;
use std::io::Read;
use std::str::Lines;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        "problem_06"
    }

    fn part1(&self) -> io::Result<i64> {
        Ok(part_impl(customs_declaration_form_anyone)? as i64)
    }

    fn part2(&self) -> io::Result<i64> {
        Ok(part_impl(customs_declaration_form_everyone)? as i64)
    }
}

fn part_impl<F>(customs_form: F) -> io::Result<usize>
where
    F: Fn(Lines) -> Vec<bool>,
{
    let mut contents = String::new();
    File::open("./data/06.txt")?.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n\n")
        .map(|s| s.lines())
        .map(|line| customs_form(line))
        .map(count_positive)
        .sum())
}

fn count_positive(vec: Vec<bool>) -> usize {
    vec.into_iter().filter(|x| *x).count()
}

fn customs_declaration_form_anyone(i: Lines) -> Vec<bool> {
    let mut base = vec![false; 26];
    for c in i.into_iter().flat_map(str::chars) {
        let idx = (c as usize) - 'a' as usize;
        base[idx] = true;
    }
    base
}

fn customs_declaration_form_everyone(i: Lines) -> Vec<bool> {
    let mut base = vec![0; 26];
    let mut people = 0;
    for line in i {
        for c in line.chars() {
            let idx = (c as usize) - 'a' as usize;
            base[idx] += 1;
        }
        people += 1;
    }
    base.into_iter().map(|x| x == people).collect()
}
