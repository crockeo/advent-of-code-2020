use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part1() -> io::Result<u32> {
    Ok(
        load_passwords()?
            .iter()
            .filter(|password| password.is_valid_sled())
            .count() as u32
    )
}

pub fn part2() -> io::Result<u32> {
    Ok(
        load_passwords()?
            .iter()
            .filter(|password| password.is_valid_toboggan())
            .count() as u32
    )
}

#[derive(Debug)]
struct Password {
    min: u32,
    max: u32,
    c: char,
    s: String,
}

impl Password {
    fn parse(input: &str) -> Option<Password> {
        lazy_static! {
            static ref PASSWORD_RE: Regex = Regex::new(
                r"(?P<min>\d+)-(?P<max>\d+) (?P<c>\w): (?P<s>\w+)"
            ).unwrap();
        }

        PASSWORD_RE.captures(input).and_then(|captures| {
            Some(
                Password {
                    min: captures.name("min")?.as_str().parse::<u32>().unwrap(),
                    max: captures.name("max")?.as_str().parse::<u32>().unwrap(),
                    c: captures.name("c")?.as_str().chars().next()?,
                    s: captures.name("s")?.as_str().to_string(),
                }
            )
        })
    }

    fn is_valid_sled(&self) -> bool {
        let mut count = 0;
        for c in self.s.chars() {
            if c == self.c {
                count += 1
            }
        }

        return self.min <= count && count <= self.max;
    }

    fn is_valid_toboggan(&self) -> bool {
        // the naming in this function is a little bit weird, because it's based on the initial
        // framing of question 2
        let chars = self.s.chars().collect::<Vec<char>>();

        let c1 = chars[self.min as usize - 1];
        let c2 = chars[self.max as usize - 1];



        (self.c == c1 || self.c == c2) && !(self.c == c1 && self.c == c2)
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password {{ {}-{} {}: {} }}", self.min, self.max, self.c, self.s)
    }
}

fn load_passwords() -> io::Result<Vec<Password>> {
    let file = File::open(Path::new(".").join("data").join("02.txt").as_path())?;

    let mut passwords = Vec::new();
    for line in BufReader::new(file).lines() {
        let real_line = line?;
        match Password::parse(&real_line) {
            None => {
                println!("failed to parse line: {}", real_line);
                continue;
            },
            Some(password) => passwords.push(password),
        }
    }
    Ok(passwords)
}
