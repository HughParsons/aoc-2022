use std::{collections::HashSet, fs, ops::BitAnd, path::Path};

const Q1_PATH: &str = "src/test2.txt";

fn fold_to_set(mut set: HashSet<char>, new: char) -> HashSet<char> {
    set.insert(new);
    set
}

fn calculate_priority(l: char) -> u32 {
    let val: u32 = l.into();
    if val < 97 {
        return val - 38;
    }
    val - 96
}

fn get_overlap_priority(row: &str) -> u32 {
    let n = row.len();
    let first = &row[..n / 2].chars().fold(HashSet::new(), fold_to_set);
    let second = &row[n / 2..].chars().fold(HashSet::new(), fold_to_set);
    let overlap = *first
        .bitand(second)
        .iter()
        .next()
        .expect("overlap should exist");
    calculate_priority(overlap)
}

fn get_badge_priority(rows: &Vec<&str>) -> u32 {
    let rows_sets = rows
        .iter()
        .map(|&r| r.chars().fold(HashSet::new(), fold_to_set))
        .reduce(|accum, cur| accum.bitand(&cur))
        .expect("Learn about all these options existing");
    let badge = *rows_sets.iter().next().expect("believe");
    calculate_priority(badge)
}

fn main() {
    let path = Path::new(Q1_PATH);
    let contents = fs::read_to_string(path).expect("Error reading file");
    let mut contents = contents.split('\n');
    let mut groups_of_three: Vec<Vec<&str>> = Vec::new();
    loop {
        let a = contents.next();
        let b = contents.next();
        let c = contents.next();

        match (a, b, c) {
            (Some(x), Some(y), Some(z)) => {
                groups_of_three.push(vec![x, y, z]);
            }
            _ => {
                break;
            }
        }
    }
    let res = groups_of_three
        .iter()
        .map(get_badge_priority)
        .reduce(|accum, cur| accum + cur)
        .expect("?");

    println!("{res:?}");
}
