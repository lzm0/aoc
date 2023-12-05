use std::fs;

type Almanac = (Vec<u32>, Vec<Vec<(u32, u32, u32)>>);

fn parse_input(input: &str) -> Almanac {
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
        if section.chars().nth(0).unwrap().is_alphabetic() {
            continue;
        }
        maps.push(
            section
                .split("\n")
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

fn part_one(almanac: &Almanac) -> u32 {
    let (seeds, maps) = almanac;
    0
}

fn part_two(almanac: &Almanac) -> u32 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-05.txt").unwrap();
    let almanac = parse_input(&input);

    println!("Part one: {}", part_one(&almanac));
    // println!("Part two: {}", part_two(&almanac));
}

#[test]
fn test_part_one() {
    let input = "seeds: 79 14 55 13

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
    let almanac = parse_input(&input);
    assert_eq!(part_one(&almanac), 35)
}

#[test]
fn test_part_two() {
    todo!()
}
