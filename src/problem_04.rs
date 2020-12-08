use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        return "problem_04";
    }

    fn part1(&self) -> io::Result<i64> {
        let mut file = File::open("./data/04.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let passports = parse_batch_passports(contents.as_str());

        Ok(passports.len() as i64)
    }

    fn part2(&self) -> io::Result<i64> {
        let mut file = File::open("./data/04.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let passports = parse_batch_passports(contents.as_str());
        Ok(passports
            .iter()
            .filter(|passport| passport.is_valid())
            .count() as i64)
    }
}

// best-effort parsing, i.e. we just include the ones that are known to be valid... for now
fn parse_batch_passports(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .filter_map(Passport::parse)
        .collect::<Vec<Passport>>()
}

enum Height {
    In(i32),
    Cm(i32),
}

impl FromStr for Height {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"(?P<amount>\d+)(?P<metric>in|cm)").unwrap();
        }

        let captures = HEIGHT_RE.captures(s).ok_or("failed to match regex")?;

        let amount = captures
            .name("amount")
            .ok_or("no amount")?
            .as_str()
            .parse::<i32>()
            .map_err(|_| "failed to parse amount")?;
        let metric = captures.name("metric").ok_or("no metric")?.as_str();

        match metric {
            "in" => Ok(Height::In(amount)),
            "cm" => Ok(Height::Cm(amount)),
            _ => Err("invalid metric"),
        }
    }
}

#[derive(Debug)]
struct Passport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    expiration_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
    country_id: Option<&'a str>,
}

impl Passport<'_> {
    // assumes that the string, s, has already been segmented into separate passports. i.e. we do
    // not want to pass in all of the batch passports here
    fn parse(s: &'_ str) -> Option<Passport> {
        let mut mappings = HashMap::new();
        for token in s.split_whitespace() {
            let mut iter = token.split(":");

            let key = iter.next()?;
            let value = iter.next()?;

            mappings.insert(key, value);
        }

        Some(Passport {
            birth_year: mappings.remove("byr")?,
            issue_year: mappings.remove("iyr")?,
            expiration_year: mappings.remove("eyr")?,
            height: mappings.remove("hgt")?,
            hair_color: mappings.remove("hcl")?,
            eye_color: mappings.remove("ecl")?,
            passport_id: mappings.remove("pid")?,
            country_id: mappings.remove("cid"),
        })
    }

    fn is_valid(&self) -> bool {
        is_birth_year_valid(self.birth_year)
            && is_issue_year_valid(self.issue_year)
            && is_expiration_year_valid(self.expiration_year)
            && is_height_valid(self.height)
            && is_hair_color_valid(self.hair_color)
            && is_eye_color_valid(self.eye_color)
            && is_passport_id_valid(self.passport_id)
    }
}

fn is_birth_year_valid(byr: &str) -> bool {
    if let Ok(byr) = byr.parse::<i32>() {
        if byr < 1920 || byr > 2002 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn is_issue_year_valid(iyr: &str) -> bool {
    if let Ok(iyr) = iyr.parse::<i32>() {
        if iyr < 2010 || iyr > 2020 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn is_expiration_year_valid(eyr: &str) -> bool {
    if let Ok(eyr) = eyr.parse::<i32>() {
        if eyr < 2020 || eyr > 2030 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn is_height_valid(hgt: &str) -> bool {
    if let Ok(hgt) = hgt.parse::<Height>() {
        match hgt {
            Height::In(amt) => {
                if amt < 59 || amt > 76 {
                    false
                } else {
                    true
                }
            }
            Height::Cm(amt) => {
                if amt < 150 || amt > 193 {
                    false
                } else {
                    true
                }
            }
        }
    } else {
        false
    }
}

fn is_hair_color_valid(hcl: &str) -> bool {
    lazy_static! {
        static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HAIR_RE.is_match(hcl)
}

fn is_eye_color_valid(ecl: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn is_passport_id_valid(pid: &str) -> bool {
    lazy_static! {
        static ref PASSPORT_PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    PASSPORT_PID_RE.is_match(pid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        let passports_str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = parse_batch_passports(passports_str);
        assert_eq!(passports.len(), 4);

        let valid_passports = passports.into_iter().filter(Passport::is_valid).count();
        assert_eq!(valid_passports, 4);
    }

    #[test]
    fn test_invalid() {
        let passports_str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passports = parse_batch_passports(passports_str);
        assert_eq!(passports.len(), 4);

        let valid_passports = passports.into_iter().filter(Passport::is_valid).count();
        assert_eq!(valid_passports, 0);
    }

    fn run_is_valid_test<F>(f: F, values: Vec<(&str, bool)>)
    where
        F: Fn(&str) -> bool,
    {
        for (s, expected_is_valid) in values.into_iter() {
            let is_valid = f(s);
            assert_eq!(
                is_valid, expected_is_valid,
                "{} expected valid: {} but is {}",
                s, expected_is_valid, is_valid,
            );
        }
    }

    #[test]
    fn test_is_valid_birth_year() {
        run_is_valid_test(is_birth_year_valid, vec![("2002", true), ("2003", false)]);
    }

    #[test]
    fn test_is_valid_height() {
        run_is_valid_test(
            is_height_valid,
            vec![
                ("60in", true),
                ("190cm", true),
                ("190in", false),
                ("190", false),
            ],
        );
    }

    #[test]
    fn test_is_valid_hair_color() {
        run_is_valid_test(
            is_hair_color_valid,
            vec![("#123abc", true), ("#123abz", false), ("123abc", false)],
        );
    }

    #[test]
    fn test_is_valid_eye_color() {
        run_is_valid_test(is_eye_color_valid, vec![("brn", true), ("wat", false)]);
    }

    #[test]
    fn test_is_valid_passport_id() {
        run_is_valid_test(
            is_passport_id_valid,
            vec![("000000001", true), ("0123456789", false)],
        );
    }
}
