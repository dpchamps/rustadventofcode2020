use RustAdventOfCode::get_resource;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

struct Passport {
    kv_pairs: Vec<(String, String)>,
}

const EXPECTED_KEYS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const EYE_COLORS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn new(entry: &String) -> Passport {
        Passport {
            kv_pairs: entry
                .split(" ")
                .map(|x| match &x.split(":").collect::<Vec<&str>>()[..] {
                    &[k, v, ..] => (String::from(k), String::from(v)),
                    _ => panic!(),
                })
                .collect(),
        }
    }

    fn has_key(&self, val: &str) -> bool {
        self.kv_pairs.iter().any(|(other_key, _)| val == other_key)
    }

    fn validate_byr(val: &str) -> bool {
        val.parse::<i32>().map_or(false, |x| x >= 1920 && x <= 2002)
    }

    fn validate_iyr(val: &str) -> bool {
        val.parse::<i32>().map_or(false, |x| x >= 2010 && x <= 2020)
    }

    fn validate_eyr(val: &str) -> bool {
        val.parse::<i32>().map_or(false, |x| x >= 2020 && x <= 2030)
    }

    fn validate_hgt(val: &str) -> bool {
        lazy_static! {
            static ref HeightMatcher: Regex = Regex::new(r"(\d{3}|\d{2})(cm|in)").unwrap();
        }

        match HeightMatcher.captures_iter(val).next() {
            Some(cap) => match cap.get(2).map(|x| x.as_str()) {
                Some("cm") => cap[1]
                    .parse::<i32>()
                    .map_or(false, |x| x >= 150 && x <= 193),
                Some("in") => cap[1].parse::<i32>().map_or(false, |x| x >= 59 && x <= 76),
                _ => false,
            },
            _ => false,
        }
    }

    fn validate_hcl(val: &str) -> bool {
        lazy_static! {
            static ref ColorMatcher: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
        }

        ColorMatcher.is_match(val) && val.len() == 7
    }

    fn validate_ecl(val: &str) -> bool {
        EYE_COLORS.iter().any(|&x| x == val)
    }

    fn validate_pid(val: &str) -> bool {
        lazy_static! {
            static ref PidMatcher: Regex = Regex::new(r"\d{9}").unwrap();
        }

        PidMatcher.is_match(val) && val.len() == 9
    }

    fn validate_val((k, v): &(String, String)) -> bool {
        match k.as_ref() {
            "byr" => Passport::validate_byr(v),
            "iyr" => Passport::validate_iyr(v),
            "eyr" => Passport::validate_eyr(v),
            "hgt" => Passport::validate_hgt(v),
            "hcl" => Passport::validate_hcl(v),
            "ecl" => Passport::validate_ecl(v),
            "pid" => Passport::validate_pid(v),
            _ => true,
        }
    }

    fn validate(&self) -> bool {
        self.kv_pairs.iter().all(Passport::validate_val)
            && EXPECTED_KEYS.iter().all(|x| self.has_key(x))
    }
}

fn get_passport_entries(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|x| String::from(x.replace("\n", " ").trim()))
        .collect()
}

fn main() {
    let input = get_resource("day4-part1");
    let valid_passports = get_passport_entries(&input)
        .iter()
        .map(Passport::new)
        .filter(|x| x.validate())
        .count();

    println!("Valid Passports: {}", valid_passports);
}

#[cfg(test)]
mod day_4_tests {
    use crate::{get_passport_entries, Passport};

    #[test]
    fn should_get_entries() {
        let input = String::from(
            "abc abc \n\
        abc \n\n\
        abc",
        );

        assert_eq!(get_passport_entries(&input), vec!["abc abc  abc", "abc"])
    }

    #[test]
    fn should_create_a_passport() {
        let entry = String::from("a:b c:d");
        let passport = Passport::new(&entry);
        let expectation = vec![
            (String::from("a"), String::from("b")),
            (String::from("c"), String::from("d")),
        ];

        assert_eq!(passport.kv_pairs, expectation)
    }

    #[test]
    fn should_validate_a_passport() {
        let entry = get_passport_entries(&String::from(
            "eyr:2028 iyr:2016 byr:1995 ecl:oth
pid:543685203 hcl:#c0946f
hgt:152cm
cid:252

eyr:2028 byr:1995 ecl:oth
pid:543685203 hcl:#c0946f
hgt:152cm
cid:252",
        ));

        let passport_valid = Passport::new(&entry[0]);
        let passport_invalid = Passport::new(&entry[1]);

        assert!(passport_valid.validate());
        assert!(!passport_invalid.validate());
    }
}
