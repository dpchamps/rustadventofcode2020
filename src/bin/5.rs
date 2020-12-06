use RustAdventOfCode::get_resource;
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct BinaryRange(i32, i32);

impl BinaryRange {
    fn split(&self) -> (i32, i32) {
        let difference = self.1 - self.0;
        let midpoint: i32 = f32::ceil((difference as f32) / 2.0) as i32;

        (self.0 + midpoint, self.1 - midpoint)
    }

    pub fn take_lower(&self) -> Self {
        BinaryRange(self.0, self.split().1)
    }

    pub fn take_higher(&self) -> Self {
        BinaryRange(self.split().0, self.1)
    }

    pub fn chain(&self, other: BinaryRange) -> Self {
        BinaryRange(self.1, other.0)
    }
}

fn reduce_rows(range: BinaryRange, el: char) -> BinaryRange {
    match el {
        'F' => range.take_lower(),
        'B' => range.take_higher(),
        _ => panic!(format!("Received invalid input {}", el)),
    }
}

fn reduce_seats(range: BinaryRange, el: char) -> BinaryRange {
    match el {
        'L' => range.take_lower(),
        'R' => range.take_higher(),
        _ => panic!(format!("Received invalid input {}", el)),
    }
}

fn compute_id(range: BinaryRange) -> i32 {
    range.0 * 8 + range.1
}

fn get_seat_id(input: &str) -> i32 {
    match input.split_at(7) {
        (rows, seats) => compute_id(
            rows.chars()
                .fold(BinaryRange(0, 127), reduce_rows)
                .chain(seats.chars().fold(BinaryRange(0, 7), reduce_seats)),
        ),
    }
}

fn get_input() -> Vec<String> {
    get_resource("day5-part1")
        .split("\n")
        .map(String::from)
        .collect()
}

fn find_max(list: &Vec<String>) -> i32 {
    list.iter().map(|x| get_seat_id(x)).fold(-1, i32::max)
}

fn extract_gaps(sorted_list: &Vec<i32>) -> Vec<(usize, i32)> {
    sorted_list
        .iter()
        .enumerate()
        .fold((vec![], -1), |(mut candidates, last), (idx, &el)| {
            if idx == 0 {
                return (candidates, el);
            }
            if el - last > 1 {
                (last + 1..el).for_each(|x| candidates.push((idx - 1, x)));
            }
            (candidates, el)
        })
        .0
}

fn find_valid_gap(list: &Vec<String>) -> Option<i32> {
    let mut sortable: Vec<i32> = list.iter().map(|x| get_seat_id(x)).collect();
    sortable.sort();

    extract_gaps(&sortable)
        .iter()
        .find(
            |(idx, el)| match (sortable.get(*idx), sortable.get(*idx + 1)) {
                (Some(x), Some(y)) => (el - x).abs() == 1 && (el - y).abs() == 1,
                _ => false,
            },
        )
        .map(|(_, el)| *el)
}

fn main() {
    let inputs = get_input();
    let result = find_max(&inputs);
    println!("Max id: {}", result);
    println!("Your seat sir: {:?}", find_valid_gap(&inputs));
}

#[cfg(test)]
mod day_5_tests {
    use crate::{find_max, get_seat_id, reduce_rows, reduce_seats, BinaryRange};

    #[test]
    fn bin_rage_take() {
        let range = BinaryRange(0, 11);

        assert_eq!(range.take_lower(), BinaryRange(0, 5));
        assert_eq!(range.take_higher(), BinaryRange(6, 11));
    }

    #[test]
    fn reduce_rows_test() {
        let range = BinaryRange(0, 127);
        let input = "FBFBBFF";
        let result = input.chars().fold(range, reduce_rows);

        assert_eq!(result, BinaryRange(44, 44));
    }

    #[test]
    fn reduce_rows_edge_cases() {
        let range = BinaryRange(0, 127);
        let input_one = "FFFFFFF";
        let result_one = input_one.chars().fold(range, reduce_rows);
        let input_two = "BBBBBBB";
        let result_two = input_two.chars().fold(range, reduce_rows);

        assert_eq!(result_one, BinaryRange(0, 0));
        assert_eq!(result_two, BinaryRange(127, 127));
    }

    #[test]
    fn reduce_seats_test() {
        let range = BinaryRange(0, 7);
        let input = "RLR";

        assert_eq!(input.chars().fold(range, reduce_seats), BinaryRange(5, 5))
    }

    #[test]
    fn test_get_seat_id() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567)
    }

    #[test]
    fn test_find_max() {
        let input = vec![
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];
        assert_eq!(find_max(&input), 820);
    }
}
