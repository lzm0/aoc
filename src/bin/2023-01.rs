use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    let file = File::open("src/input/2023-01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let content = line.unwrap();
        let first_digit = content
            .chars()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = content
            .chars()
            .rfind(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += 10 * first_digit + last_digit;
    }
    sum
}

fn part_two() -> u32 {
    let file = File::open("src/input/2023-01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let content = line.unwrap();
    }
    sum
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
