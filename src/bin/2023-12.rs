type Record = (Vec<char>, Vec<usize>);

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let springs = parts.0.chars().collect::<Vec<_>>();
            let counts = parts
                .1
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            (springs, counts)
        })
        .collect()
}

fn count_arrangements(record: &Record) -> usize {
    let (springs, counts) = record;
    fn backtrack(springs: &[char], counts: &[usize], group_i: usize, group_len: usize) -> usize {
        let mut arrangements = 0;
        dbg!(&springs, &counts, group_i, group_len,);
        if springs.is_empty() {
            if group_len > 0 && group_len == counts[group_i] {
                arrangements += 1;
                return arrangements;
            }
            return 0;
        }
        match springs[0] {
            '?' => {
                // Case `#`
                if group_i < counts.len() {
                    arrangements += backtrack(&springs[1..], counts, group_i, group_len + 1);
                }
                // Case `.`
                if group_len > 0 && group_len == counts[group_i] {
                    arrangements += backtrack(&springs[1..], counts, group_i + 1, 0);
                } else {
                    arrangements += backtrack(&springs[1..], counts, group_i, 0);
                }
            }
            '#' => {
                if group_len == counts[group_i] {
                    // When the current group length is longer than it should be
                    return 0;
                }
                arrangements += backtrack(&springs[1..], counts, group_i, group_len + 1);
            }
            '.' => {
                if group_len > 0 {
                    if group_len == counts[group_i] {
                        arrangements += backtrack(&springs[1..], counts, group_i + 1, 0);
                    } else {
                        return 0;
                    }
                } else {
                    arrangements += backtrack(&springs[1..], counts, group_i, 0);
                }
            }
            _ => unreachable!(),
        }
        arrangements
    }
    backtrack(springs, counts, 0, 0)
}

fn get_contiguous_groups(chars: &[char]) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut count = 0;
    for &c in chars {
        if c == '#' {
            count += 1;
        } else if count > 0 {
            groups.push(count);
            count = 0;
        }
    }
    if count > 0 {
        groups.push(count);
    }
    groups
}

fn part_one(records: &[Record]) -> usize {
    records.iter().map(count_arrangements).sum()
}

fn part_two(records: &[Record]) -> usize {
    records
        .iter()
        .map(|(springs, counts)| {
            let left = springs
                .iter()
                .chain(['?'].iter())
                .copied()
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect::<Vec<_>>();
            let right = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect::<Vec<_>>();
            left.iter().for_each(|c| print!("{}", c));
            println!();
            count_arrangements(&(left, right))
        })
        .sum()
}

fn main() {
    let records = parse(include_str!("../input/2023-12.txt"));

    println!("Part one: {}", part_one(&records));
    println!("Part two: {}", part_two(&records));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_count_arrangements() {
        let record = &parse("?###???????? 3,2,1")[0];
        assert_eq!(count_arrangements(record), 10);
    }

    #[test]
    fn test_count_arrangements_simple() {
        let record = &parse("???.### 1,1,3")[0];
        assert_eq!(count_arrangements(record), 1);
    }

    #[test]
    fn test_count_arrangements_very_simple() {
        let record = &parse("### 3")[0];
        assert_eq!(count_arrangements(record), 1);
    }

    #[test]
    fn test_get_contiguous_groups() {
        let chars = ".###.##....#".chars().collect::<Vec<_>>();
        assert_eq!(get_contiguous_groups(&chars), vec![3, 2, 1]);
    }

    const EXAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
    };

    #[test]
    fn test_part_one() {
        let records = parse(EXAMPLE);
        assert_eq!(part_one(&records), 21);
    }

    #[test]
    fn test_part_two() {
        let records = parse(EXAMPLE);
        assert_eq!(part_two(&records), 525152);
    }

    #[test]
    fn test_part_two_simple() {
        let records = parse("????.######..#####. 1,6,5");
        assert_eq!(part_two(&records), 2500);
    }
}
