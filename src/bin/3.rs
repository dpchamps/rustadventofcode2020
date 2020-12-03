use RustAdventOfCode::get_resource;

fn get_map() -> Vec<Vec<char>> {
    get_resource("day3-part1")
        .split("\n")
        .map(|s| s.chars().collect())
        .collect()
}

fn map_tree(x: &char) -> i32 {
    match x {
        '#' => 1,
        _ => 0,
    }
}

fn get_idx(row_num: usize, right: usize) -> usize {
    return (row_num + 1) * right;
}

fn extract_from_col(col: &Vec<char>, idx: usize) -> i32 {
    col.get(idx % col.len()).map(map_tree).unwrap_or(0)
}

fn tree_counter_part_one(trees: i32, (row_num, col): (usize, &Vec<char>)) -> i32 {
    let idx = get_idx(row_num, 3) % col.len();

    trees + col.get(idx).map(map_tree).unwrap()
}

fn tree_counter_part_two(trees: Vec<i32>, (row_num, col): (usize, &Vec<char>)) -> Vec<i32> {
    trees
        .iter()
        .enumerate()
        .map(|(idx, n)| {
            n + match idx {
                0 => extract_from_col(col, get_idx(row_num, 1)),
                1 => extract_from_col(col, get_idx(row_num, 3)),
                2 => extract_from_col(col, get_idx(row_num, 5)),
                3 => extract_from_col(col, get_idx(row_num, 7)),
                4 if row_num > 0 && (row_num + 1) % 2 == 0 => {
                    extract_from_col(col, get_idx(row_num / 2, 1))
                }
                _ => 0,
            }
        })
        .collect()
}

fn main() {
    let map = get_map();
    let result1 = map
        .iter()
        .skip(1)
        .enumerate()
        .fold(0, tree_counter_part_one);

    println!("Trees: {}", result1);

    let result2 = map
        .iter()
        .skip(1)
        .enumerate()
        .fold(vec![0, 0, 0, 0, 0], tree_counter_part_two)
        .into_iter()
        .fold(1 as i64, |product, val| product * val as i64);

    println!("More Trees: {:?}", result2);
}
