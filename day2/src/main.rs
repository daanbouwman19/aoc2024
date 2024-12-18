use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("robin.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let data: Vec<Vec<isize>> = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|str| str.parse().unwrap())
                .collect()
        })
        .collect();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &[Vec<isize>]) {
    let amount_valid = data.iter().filter(|line| validate_line(line)).count();
    println!("part one: safe amount: {}", amount_valid);
}

fn part_two(data: &[Vec<isize>]) {
    let amount_valid = data
        .iter()
        .filter(|line| validate_line_with_remove(line))
        .count();
    println!("part two: safe amount: {}", amount_valid);
}

fn validate_line(line: &[isize]) -> bool {
    let window = line.windows(2);

    let diffs: Vec<isize> = window.map(|pair| pair[1] - pair[0]).collect();

    let in_bounds = diffs.iter().all(|&diff| (1..=3).contains(&diff.abs()));
    let increasing = diffs.iter().all(|&diff| diff > 0);
    let decreasing = diffs.iter().all(|&diff| diff < 0);

    in_bounds && (increasing || decreasing)
}

fn validate_line_with_remove(line: &[isize]) -> bool {
    for i in 0..line.len() {
        if validate_line(
            &line
                .iter()
                .enumerate()
                .filter_map(|(index, &value)| if index == i { None } else { Some(value) })
                .collect::<Vec<isize>>(),
        ) {
            return true;
        }
    }
    false
}
