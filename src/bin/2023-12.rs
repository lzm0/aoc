type Record = (Vec<char>, Vec<usize>);

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let chars = parts.0.chars().collect::<Vec<_>>();
            let nums = parts
                .1
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            (chars, nums)
        })
        .collect()
}

fn count_arrangements(record: &Record) -> usize {
    let &(chars, nums) = record;
    todo!()
}

fn get_contiguous_groups(chars: &[char]) -> Vec<usize> {
    let mut groups = Vec::new();
    todo!()
}

fn part_one(records: &[Record]) -> usize {
    records.iter().map(count_arrangements).sum()
}

fn main() {
    let records = parse(include_str!("../input/2023-12.txt"));

    println!("Part one: {}", part_one(&records));
    // println!("Part two: {}", part_two(&records));
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
    fn test_part_one() {
        let records = parse(indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1"
        });
        assert_eq!(part_one(&records), 21);
    }

    #[test]
    fn test_part_two() {}
}
