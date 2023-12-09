use std::fs;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(histories: &Vec<Vec<i64>>) -> i64 {
    histories
        .iter()
        .map(|history| extrapolate(history.as_slice()))
        .sum()
}

fn part_two(histories: &Vec<Vec<i64>>) -> i64 {
    histories
        .iter()
        .map(|history| {
            let mut copy = history.to_owned();
            copy.reverse();
            extrapolate(&copy)
        })
        .sum()
}

fn extrapolate(history: &[i64]) -> i64 {
    let mut arr = history.to_owned();

    let mut i = history.len() - 1;
    while i > 0 {
        let mut all_zero = true;
        for j in 0..i {
            arr[j] = arr[j + 1] - arr[j];
            if arr[j] != 0 {
                all_zero = false;
            }
        }
        if all_zero {
            break;
        }
        i -= 1;
    }

    arr[i..].iter().sum()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-09.txt").unwrap();

    println!("Part one: {}", part_one(&parse(&input)));
    println!("Part two: {}", part_two(&parse(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &'static str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 2);
    }
}
