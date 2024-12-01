use std::{collections::HashMap, io::BufRead};

const EXAMPLE_FILE: &str = "example.txt";
const INPUT_FILE: &str = "input.txt";

fn main() {
    part_one(EXAMPLE_FILE);
    part_one(INPUT_FILE);

    part_two(EXAMPLE_FILE);
    part_two(INPUT_FILE);
}

fn part_one(filename: &str) {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_ascii_whitespace();
            let num_left: i32 = parts.next().unwrap().parse().unwrap();
            let num_right: i32 = parts.next().unwrap().parse().unwrap();
            (num_left, num_right)
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Part 1. File: {}, output: {}", filename, total_distance);
}

fn part_two(filename: &str) {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let (left, right): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_ascii_whitespace();
            let left: i32 = parts.next().unwrap().parse().unwrap();
            let right: i32 = parts.next().unwrap().parse().unwrap();

            (left, right)
        })
        .unzip();

    let right_counted: HashMap<i32, i32> = right.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let total: i32 = left
        .iter()
        .filter_map(|value| right_counted.get(value).map(|count| value * count))
        .sum();

    println!("Part 2. File: {}, output: {}", filename, total);
}
