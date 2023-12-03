use std::{collections::HashMap, fs};

fn part_one(input: &Vec<String>) -> u32 {
    let m = input.len();
    let n = input[0].len();

    let adjacent_to_symbol = |i, j| -> bool {
        let offsets = [
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        offsets.iter().any(|(di, dj)| {
            let ii = i as i32 + di;
            let jj = j as i32 + dj;
            if !(0 <= ii && ii < m as i32) || !(0 <= jj && jj < n as i32) {
                return false;
            }
            is_symbol(input[ii as usize].chars().nth(jj as usize).unwrap())
        })
    };

    let mut sum = 0;
    for i in 0..m {
        let row = &input[i];
        let mut j = 0;
        let mut is_part = false;
        let mut part_number = 0;
        while j < n {
            let c = row.chars().nth(j).unwrap();
            if c.is_digit(10) {
                if adjacent_to_symbol(i, j) {
                    is_part = true;
                }
                part_number = part_number * 10 + c.to_digit(10).unwrap();
            } else {
                if is_part {
                    sum += part_number;
                }
                part_number = 0;
                is_part = false;
            }
            j += 1;
        }
        if is_part {
            sum += part_number;
        }
    }
    sum
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn part_two(input: &Vec<String>) -> u32 {
    let m = input.len();
    let n = input[0].len();

    let mut sum = 0;

    for i in 0..m {
        for j in 0..n {
            let mut part_number_count = 0;
            let mut gear_ratio = 1;
            if input[i].chars().nth(j).unwrap() != '*' {
                continue;
            }
            let offsets = [
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ];

            let mut visited = HashMap::new();
            offsets.iter().for_each(|(i, j)| {
                visited.insert((*i, *j), false);
            });

            for (di, dj) in offsets {
                if *visited.get(&(di, dj)).unwrap() {
                    continue;
                }
                let ii = i as i32 + di;
                let jj = j as i32 + dj;
                if !(0 <= ii && ii < m as i32)
                    || !(0 <= jj && jj < n as i32)
                    || !input[ii as usize]
                        .chars()
                        .nth(jj as usize)
                        .unwrap()
                        .is_digit(10)
                {
                    continue;
                }

                let mut left = jj;
                while left - 1 >= 0
                    && input[ii as usize]
                        .chars()
                        .nth((left - 1) as usize)
                        .unwrap()
                        .is_digit(10)
                {
                    left -= 1;
                    if visited.contains_key(&(di, left - j as i32)) {
                        visited.insert((di, left - j as i32), true);
                    }
                }

                let mut right = jj;
                while right + 1 < n as i32
                    && input[ii as usize]
                        .chars()
                        .nth((right + 1) as usize)
                        .unwrap()
                        .is_digit(10)
                {
                    right += 1;
                    if visited.contains_key(&(di, right - j as i32)) {
                        visited.insert((di, right - j as i32), true);
                    }
                }

                part_number_count += 1;
                if part_number_count <= 2 {
                    gear_ratio *= input[ii as usize][left as usize..(right + 1) as usize]
                        .parse::<u32>()
                        .unwrap();
                }
            }
            if part_number_count == 2 {
                sum += gear_ratio;
            }
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("src/input/2023-03.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .lines()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(part_one(&input), 4361);
}

#[test]
fn test_part_two() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .lines()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(part_two(&input), 467835);
}
