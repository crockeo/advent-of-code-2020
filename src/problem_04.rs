use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn part1() -> io::Result<i32> {
    let mut file = File::open("./data/04.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let passports = parse_batch_passports(contents.as_str());

    Ok(passports.len() as i32)
}

// best-effort parsing, i.e. we just include the ones that are known to be valid... for now
fn parse_batch_passports(s: &str) -> Vec<Passport> {
    s.split("\n\n").filter_map(Passport::parse).collect::<Vec<Passport>>()
}

struct Passport {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: i32,
    country_id: Option<i32>,
}

impl Passport {
    // assumes that the string, s, has already been segmented into separate passports. i.e. we do
    // not want to pass in all of the batch passports here
    fn parse(s: &str) -> Option<Passport> {
        let mut mappings = HashMap::new();
        for token in s.split_whitespace() {
            let mut iter = token.split(":");

            let key = iter.next()?;
            let value = iter.next()?;

            mappings.insert(key, value);
        }

        Some(
            Passport {
                birth_year: mappings.get("byr")?.parse::<i32>().unwrap(),
                issue_year: mappings.get("iyr")?.parse::<i32>().unwrap(),
                expiration_year: mappings.get("eyr")?.parse::<i32>().unwrap(),
                height: mappings.get("hgt")?.to_string(),
                hair_color: mappings.get("hcl")?.to_string(),
                eye_color: mappings.get("ecl")?.to_string(),
                passport_id: mappings.get("pid")?.parse::<i32>().unwrap(),
                country_id: mappings.get("cid").map(|cid| cid.parse::<i32>().unwrap()),
            }
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let example = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let passports = parse_batch_passports(example);
        assert_eq!(passports.len(), 2);
        assert_eq!(passports[0].passport_id, 860033327);
        assert_eq!(passports[1].passport_id, 760753108);
    }
}
