use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_two(lines: &[String]) {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut total = 0;
    let mut execute_mul = true;

    for line in lines {
        for cap in re.captures_iter(line) {
            match cap.get(0).unwrap().as_str() {
                "do()" => execute_mul = true,
                "don't()" => execute_mul = false,
                _ if execute_mul => {
                    let value1: isize = cap[2].parse().unwrap();
                    let value2: isize = cap[3].parse().unwrap();
                    total += value1 * value2;
                }
                _ => continue,
            }
        }
    }

    println!("part two: {}", total);
}

fn part_one(lines: &[String]) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for line in lines {
        for result in re.captures_iter(line) {
            let value1: isize = result[1].parse().unwrap();
            let value2: isize = result[2].parse().unwrap();
            total += value1 * value2;
        }
    }

    println!("part one: {}", total);
}
