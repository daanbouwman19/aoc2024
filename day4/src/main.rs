use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let data: Vec<Vec<char>> = buf_reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &[Vec<char>]) {
    let word = "XMAS";
    let mut count = 0;

    for (y, line) in data.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            count += [
                find_word_x_axis(data, word, x, y, false),
                find_word_x_axis(data, word, x, y, true),
                find_word_y_axis(data, word, x, y, false),
                find_word_y_axis(data, word, x, y, true),
                find_word_diagonal_1(data, word, x, y, false),
                find_word_diagonal_1(data, word, x, y, true),
                find_word_diagonal_2(data, word, x, y, false),
                find_word_diagonal_2(data, word, x, y, true),
            ]
            .iter()
            .filter(|&x| *x)
            .count();
        }
    }

    println!("count: {}", count);
}

fn part_two(data: &[Vec<char>]) {
    let word = "MAS";
    let mut count = 0;

    for (y, line) in data.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            count += [
                find_word_diagonal_1(data, word, x, y, false)
                    && find_word_diagonal_2(data, word, x + 2, y, false),
                find_word_diagonal_1(data, word, x, y, true)
                    && find_word_diagonal_2(data, word, x + 2, y, false),
                find_word_diagonal_1(data, word, x, y, false)
                    && find_word_diagonal_2(data, word, x + 2, y, true),
                find_word_diagonal_1(data, word, x, y, true)
                    && find_word_diagonal_2(data, word, x + 2, y, true),
            ]
            .iter()
            .filter(|&x| *x)
            .count();
        }
    }

    println!("count: {}", count);
}

fn get_reverse_index(reversed: bool, index: usize, length: usize) -> usize {
    if reversed {
        length - 1 - index
    } else {
        index
    }
}

fn find_word_x_axis(data: &[Vec<char>], word: &str, x: usize, y: usize, reversed: bool) -> bool {
    if word.len() + x > data[y].len() {
        return false;
    }
    for index in 0..word.len() {
        let ch_index = get_reverse_index(reversed, index, word.len());
        if word.chars().nth(ch_index).unwrap() != data[y][x + index] {
            return false;
        }
    }
    true
}

fn find_word_y_axis(data: &[Vec<char>], word: &str, x: usize, y: usize, reversed: bool) -> bool {
    if word.len() + y > data.len() {
        return false;
    }
    for index in 0..word.len() {
        let ch_index = get_reverse_index(reversed, index, word.len());
        if word.chars().nth(ch_index).unwrap() != data[y + index][x] {
            return false;
        }
    }
    true
}

fn find_word_diagonal_1(
    data: &[Vec<char>],
    word: &str,
    x: usize,
    y: usize,
    reversed: bool,
) -> bool {
    if x + word.len() > data[y].len() || y + word.len() > data.len() {
        return false;
    }

    for index in 0..word.len() {
        let ch_index = get_reverse_index(reversed, index, word.len());
        if word.chars().nth(ch_index).unwrap() != data[y + index][x + index] {
            return false;
        }
    }
    true
}

fn find_word_diagonal_2(
    data: &[Vec<char>],
    word: &str,
    x: usize,
    y: usize,
    reversed: bool,
) -> bool {
    if x < word.len() - 1 || y + word.len() > data.len() {
        return false;
    }

    for index in 0..word.len() {
        let ch_index = get_reverse_index(reversed, index, word.len());
        if word.chars().nth(ch_index).unwrap() != data[y + index][x - index] {
            return false;
        }
    }
    true
}
