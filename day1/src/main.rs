use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let data = buf_reader
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

fn part_one(data: &Vec<Vec<isize>>) {
    let mut amount_valid = 0;
    for line in data {
        let mut increasing = true;
        let mut decreasing = true;
        let mut valid = true;

        for n in 1..line.len() {
            let current = line[n];
            let previous = line[n - 1];

            let diff = current - previous;

            if diff.abs() <= 0 || diff.abs() > 3 {
                valid = false;
            }

            if diff < 0 {
                increasing = false;
            } else if diff > 0 {
                decreasing = false;
            }
        }

        if valid && (increasing || decreasing) {
            amount_valid = amount_valid + 1;
        }
    }

    println!("part one: safe amount: {}", amount_valid);
}

fn part_two(data: &Vec<Vec<isize>>) {
    let mut amount_valid = 0;

    for line in data {
        if validate_line(line, false) {
            amount_valid = amount_valid + 1;
        }
    }

    println!("part two: safe amount: {}", amount_valid);
}

fn validate_line(line: &Vec<isize>, r: bool) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid = true;

    for n in 1..line.len() {
        let current = line[n];
        let previous = line[n - 1];

        let diff = current - previous;

        if diff.abs() <= 0 || diff.abs() > 3 {
            valid = false;
            break;
        }

        if diff < 0 {
            increasing = false;
        } else if diff > 0 {
            decreasing = false;
        }
        if !increasing && !decreasing {
            valid = false;

            break;
        }
    }

    if valid {
        return true;
    } else if !r {
        for i in 0..line.len() {
            let mut new_line = line.clone();
            new_line.remove(i);

            if validate_line(&new_line, true) {
                return true;
            }
        }
    }

    false
}
