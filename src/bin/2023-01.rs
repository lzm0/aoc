use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;

fn part_one(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let first_digit = line
            .chars()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rfind(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += 10 * first_digit + last_digit;
    }
    sum
}

fn part_two(lines: Vec<String>) -> u32 {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut sum = 0;
    for line in lines {
        for i in 0..line.len() {
            let slice = line.get(i..).unwrap();
            if let Some(number) = match_number(&map, slice) {
                sum += 10 * number;
                break;
            }
        }
        for i in (0..line.len()).rev() {
            let slice = line.get(i..).unwrap();
            if let Some(number) = match_number(&map, slice) {
                sum += number;
                break;
            }
        }
    }
    sum
}

fn match_number(map: &HashMap<&str, u32>, slice: &str) -> Option<u32> {
    for key in map.keys() {
        if slice.starts_with(key) {
            return Some(map.get(key).unwrap().to_owned());
        }
    }
    None
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("src/input/2023-01.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    println!("{}", part_one(lines.to_owned()));
    println!("{}", part_two(lines.to_owned()));
}

#[test]
fn test_part_two() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    assert_eq!(part_two(input), 281);
}
