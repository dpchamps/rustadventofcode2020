use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use RustAdventOfCode::get_resource;

fn get_answers(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|form| form.replace("\n", ""))
        .collect()
}

fn get_answers_part2(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|form| form.split("\n").map(|x| String::from(x)).collect())
        .collect()
}

fn part_one(forms: Vec<String>) -> i32 {
    forms
        .iter()
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .fold(HashMap::<char, i32>::new(), |mut state, chars| {
            chars.iter().for_each(|c| {
                match state.get(c) {
                    Some(x) => state.insert(*c, x + 1),
                    _ => state.insert(*c, 1),
                };
            });

            state
        })
        .values()
        .sum()
}

fn part_two(forms: Vec<Vec<String>>) -> i32 {
    forms
        .iter()
        .map(|answers| {
            answers
                .iter()
                .map(|x|x.chars())
                .fold(HashMap::<char, i32>::new(), |mut state, chars| {
                    chars.for_each(|c| {
                        match state.get(&c) {
                            Some(x) => state.insert(c, x + 1),
                            _ => state.insert(c, 1),
                        };
                    });

                    state
                })
                .values()
                .map(|count| return if *count == answers.len() as i32 { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum()
}

fn main() {
    println!(
        "Yes answers: {}",
        part_one(get_answers(&get_resource("day6-part1")))
    );

    println!(
        "All Yes answers: {}",
        part_two(get_answers_part2(&get_resource("day6-part1")))
    );
}
