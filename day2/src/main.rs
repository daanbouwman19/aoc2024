use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let file = File::open(FILENAME).unwrap();
    let buf_reader = BufReader::new(file);
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let mut total = 0;
    let mut execute_mul = true;

    for line in buf_reader.lines() {
        let line = line.unwrap();

        for re_match in re.find_iter(&line) {
            let re_match = re_match.as_str();
            if re_match == "do()" {
                execute_mul = true;
            } else if re_match == "don't()" {
                execute_mul = false;
            } else if execute_mul {
                let result = re_match.trim_start_matches("mul(").trim_end_matches(")");
                let values: Vec<String> = result.split(",").map(|s| s.to_string()).collect();
                let value1: i32 = values[0].parse().unwrap();
                let value2: i32 = values[1].parse().unwrap();
                total += value1 * value2;
            }
        }
    }
    println!("part two: {}", total);
}

fn part_one() {
    let file = File::open(FILENAME).unwrap();
    let buf_reader = BufReader::new(file);
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut total = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let results = re.find_iter(&line);

        for result in results {
            let mut result = result.as_str().trim_start_matches("mul(");
            result = result.trim_end_matches(")");
            let values: Vec<String> = result.split(",").map(|s| s.to_string()).collect();
            let value1: i32 = values[0].parse().unwrap();
            let value2: i32 = values[1].parse().unwrap();
            total += value1 * value2;
        }
    }

    println!("part one: {}", total);
}
