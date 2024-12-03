use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
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

    println!("{}", total);
}