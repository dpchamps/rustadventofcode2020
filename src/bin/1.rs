use itertools::izip;
use tuple_conv::*;
use RustAdventOfCode::get_resource;

fn get_numbers(file: &str) -> Vec<i32> {
    get_resource(file)
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn groups(list: Vec<i32>) -> Vec<Vec<i32>> {
    let mut groups: Vec<Vec<i32>> = vec![];

    for (idx, el) in list.iter().enumerate() {
        for x in list.iter().skip(idx + 1) {
            for y in list.iter().skip(idx + 2) {
                groups.push(vec![*el, *x, *y])
            }
        }
    }

    groups
}

fn find_by_sum(group: &&Vec<i32>) -> bool {
    group.iter().sum::<i32>() == 2020
}

fn product_list(list: &Vec<i32>) -> (i32, Vec<i32>) {
    (list.iter().fold(1, |result, &x| result * x), list.clone())
}

fn part_one() -> Option<(i32, Vec<i32>)> {
    let numbers: Vec<i32> = get_numbers("day1-part1");

    groups(numbers).iter().find(find_by_sum).map(product_list)
}

// fn part_two() -> Option<(i32, Vec<i32>)> {
//     let numbers: Vec<i32> = get_numbers("day1-part1");
//
//     groups(3, numbers)
//         .iter()
//         .find(find_by_sum)
//         .map(product_list)
// }

fn main() {
    if let Some((result, list)) = part_one() {
        println!("{}, {:?}", result, list)
    } else {
        println!(":shrug:")
    }
}
