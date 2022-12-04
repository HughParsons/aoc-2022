use std::{fs, path::Path};

const INPUT_PATH: &str = "src/input.txt";

fn convert_interval_to_tuple(interval: &str) -> (i32, i32) {
    let mut temp = interval.split('-');
    let (a, b) = (temp.next(), temp.next_back());
    match (a, b) {
        (Some(a), Some(b)) => (
            a.parse().expect("should be a valid number"),
            b.parse().expect("should be a valid number"),
        ),
        _ => (-1, -1),
    }
}

#[allow(dead_code)]
fn does_contain((a, b): (i32, i32), (x, y): (i32, i32)) -> bool {
    if y - x > b - a {
        return does_contain((x, y), (a, b));
    }
    a <= x && b >= y
}

fn does_overlap((a, b): (i32, i32), (x, y): (i32, i32)) -> bool {
    if y - x > b - a {
        return does_overlap((x, y), (a, b));
    }
    x <= b && y >= a
}

fn double_interval(row: std::str::Split<char>) -> u32 {
    let mut temp = row.map(convert_interval_to_tuple);
    let (int1, int2) = (temp.next(), temp.next_back());
    match (int1, int2) {
        (Some(a), Some(b)) => u32::from(does_overlap(a, b)),
        _ => 0,
    }
}

fn main() {
    let path = Path::new(INPUT_PATH);
    let contents = fs::read_to_string(path).expect("Error reading file");
    let res = contents
        .split('\n')
        .map(|s| s.split(','))
        .fold(0, |accum, cur| accum + double_interval(cur));

    println!("{res:?}");
}
