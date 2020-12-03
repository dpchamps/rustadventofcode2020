use regex::Regex;
use RustAdventOfCode::get_resource;

struct PasswordPolicy {
    range: (usize, usize),
    character: char,
    password: String,
}

impl PasswordPolicy {
    pub fn validate(&self) -> bool {
        let is_valid_left = self.password.chars().nth(self.range.0 - 1).unwrap() == self.character;
        let is_valid_right = self.password.chars().nth(self.range.1 - 1).unwrap() == self.character;

        is_valid_right ^ is_valid_left
    }
}

fn get_passwords(file: &str) -> Vec<PasswordPolicy> {
    let r = Regex::new(r"(\d+)-(\d+)\s(.):\s(.+)").unwrap();

    r.captures_iter(&get_resource(file))
        .map(|cap| PasswordPolicy {
            range: (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ),
            character: String::from(&cap[3]).chars().next().unwrap(),
            password: String::from(&cap[4]),
        })
        .collect()
}

fn main() {
    let valid_passwords = get_passwords("day2-part1")
        .iter()
        .filter(|x| x.validate())
        .count();
    println!("{}", valid_passwords)
}
