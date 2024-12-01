use std::{collections::HashMap, io::BufRead};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = std::fs::File::open("input.txt").unwrap();
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

    println!("part 1: {}", total_distance);
}

fn part_two() {
    let file = std::fs::File::open("input.txt").unwrap();
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

    println!("part 2: {}", total);
}
