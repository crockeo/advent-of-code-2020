use std::collections::HashMap;
use std::io;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part1() -> io::Result<i32> {
    Ok(0)
}

enum Length {
    In(i32),
    Cm(i32),
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// best-effort parsing, i.e. we just include the ones that are known to be valid... for now
fn parse_batch_passports(s: &str) -> Vec<Passport> {
    // TODO: this is SUPEr ugly, is there not a way I can just like...filter and un-option the passports at once?
    s
        .split("\n\n")
        .map(Passport::parse)
        .filter(|passport| match passport { None => false, _ => true })
        .map(|passport| passport.unwrap())
        .collect::<Vec<Passport>>()
}

struct Passport {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: Length,
    hair_color: Color,
    eye_color: String,
    passport_id: i32,
    country_id: Option<i32>,
}

impl Passport {
    // assumes that the string, s, has already been segmented into separate passports. i.e. we do
    // not want to pass in all of the batch passports here
    fn parse(s: &str) -> Option<Passport> {
        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new(
                r"(?P<key>\w+):(?P<value>\S+)",
            ).unwrap();
        }

        let mut mappings = HashMap::new();
        for token in s.split_whitespace() {
            let iter = token.split(":");

            let key = iter.next()?;
            let value = iter.next()?;

            mappings.insert(key, value);
        }

        Some(
            Passport {
                birth_year: mappings.get("byr")?.parse::<i32>().unwrap(),
                issue_year: mappings.get("iyr")?.parse::<i32>().unwrap(),
                // TODO: continue for the following
                // eyr (Expiration Year)
                // hgt (Height)
                // hcl (Hair Color)
                // ecl (Eye Color)
                // pid (Passport ID)
                country_id: mappings.get("cid").map(|cid| cid.parse::<i32>().unwrap()),
            }
        )
    }
}
