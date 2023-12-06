use std::fs;

type Almanac = (Vec<u64>, Vec<Vec<(u64, u64, u64)>>);

fn parse(input: &str) -> Almanac {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps = Vec::new();

    while let Some(section) = sections.next() {
        maps.push(
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let mut iter = line.split_whitespace();
                    (
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                    )
                })
                .collect(),
        );
    }

    (seeds, maps)
}

fn part_one(almanac: &Almanac) -> u64 {
    let (seeds, maps) = almanac;
    seeds
        .iter()
        .map(|&seed| {
            let mut num = seed;
            for map in maps {
                num = map
                    .iter()
                    .find(|(_, src, len)| src <= &num && num < src + len)
                    .map(|(dst, src, _)| dst + &num - src)
                    .unwrap_or(num);
            }
            num
        })
        .min()
        .unwrap()
}

fn part_two(almanac: &Almanac) -> u64 {
    let (seeds, maps) = almanac;
    seeds
        .chunks(2)
        .map(|range| {
            let start = range[0];
            let len = range[1];
            start..(start + len)
        })
        .flatten()
        .map(|seed| {
            let mut num = seed;
            for map in maps {
                num = map
                    .iter()
                    .find(|(_, src, len)| src <= &num && num < src + len)
                    .map(|(dst, src, _)| dst + &num - src)
                    .unwrap_or(num);
            }
            num
        })
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-05.txt").unwrap();
    let almanac = parse(&input);

    println!("Part one: {}", part_one(&almanac));
    println!("Part two: {}", part_two(&almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        let almanac = parse(TEST_INPUT);
        assert_eq!(part_one(&almanac), 35)
    }

    #[test]
    fn test_part_two() {
        let almanac = parse(TEST_INPUT);
        assert_eq!(part_two(&almanac), 46)
    }
}
