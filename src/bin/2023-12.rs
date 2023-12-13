use std::collections::HashMap;

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

    let mut spring_clone = springs.clone();
    spring_clone.push('.');

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    fn backtrack(
        springs: &[char],
        counts: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if counts.is_empty() {
            return if springs.contains(&'#') { 0 } else { 1 };
        }
        if springs.len() < counts.iter().sum::<usize>() + counts.len() {
            return 0;
        }
        if let Some(&cached) = cache.get(&(springs.len(), counts.len())) {
            return cached;
        }
        let mut arangements = 0;
        if springs[0] != '#' {
            arangements += backtrack(&springs[1..], counts, cache);
        }
        let next_group_size = counts[0];
        if !springs[..next_group_size].contains(&'.') && springs[next_group_size] != '#' {
            arangements += backtrack(&springs[next_group_size + 1..], &counts[1..], cache);
        }
        cache.insert((springs.len(), counts.len()), arangements);
        arangements
    }

    backtrack(&spring_clone, counts, &mut cache)
}

fn part_one(records: &[Record]) -> usize {
    records.iter().map(count_arrangements).sum()
}

fn part_two(records: &[Record]) -> usize {
    records
        .iter()
        .map(|(springs, counts)| {
            let springs = springs
                .iter()
                .chain(['?'].iter())
                .copied()
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect::<Vec<_>>();
            let counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect::<Vec<_>>();
            count_arrangements(&(springs, counts))
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
