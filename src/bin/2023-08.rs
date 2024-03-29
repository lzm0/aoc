use std::{collections::HashMap, fs};

enum Direction {
    L,
    R,
}
type Node = [char; 3];
type Instruction = Vec<Direction>;
type Network = HashMap<Node, [Node; 2]>;

fn parse(input: &str) -> (Instruction, Network) {
    let mut iter = input.lines();
    let instruction = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let network = iter.skip(1).fold(HashMap::new(), |mut map, line| {
        let (key, value) = line.split_once(" = ").unwrap();
        let key = key.chars().collect::<Vec<_>>().try_into().unwrap();
        let (left, right) = value[1..value.len() - 1]
            .split_once(", ")
            .map(|(left, right)| {
                (
                    left.chars().collect::<Vec<_>>().try_into().unwrap(),
                    right.chars().collect::<Vec<_>>().try_into().unwrap(),
                )
            })
            .unwrap();

        map.insert(key, [left, right]);
        map
    });
    (instruction, network)
}

fn part_one((instruction, network): &(Instruction, Network)) -> usize {
    let mut current = ['A', 'A', 'A'];
    let mut count = 0;

    while current != ['Z', 'Z', 'Z'] {
        for direction in instruction {
            let &[left, right] = network.get(&current).unwrap();
            current = match direction {
                Direction::L => left,
                Direction::R => right,
            };
            count += 1;
            if current == ['Z', 'Z', 'Z'] {
                break;
            }
        }
    }

    count
}

fn part_two((instruction, network): &(Instruction, Network)) -> usize {
    let starts = network.keys().filter(|&[_, _, x]| *x == 'A').map(|&x| x);

    let counts = starts
        .map(|x| {
            let mut count = 0;
            let mut current = x.clone();
            while current[2] != 'Z' {
                for direction in instruction {
                    let &[left, right] = network.get(&current).unwrap();
                    current = match direction {
                        Direction::L => left,
                        Direction::R => right,
                    };
                    count += 1;
                    if current[2] == 'Z' {
                        break;
                    }
                }
            }
            count
        })
        .collect::<Vec<_>>();

    lcm_all(&counts)
}

fn lcm_all(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let input = fs::read_to_string("src/input/2023-08.txt").unwrap();

    println!("Part one: {}", part_one(&parse(&input)));
    println!("Part two: {}", part_two(&parse(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_ONE: &'static str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE_TWO: &'static str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_part_one_example_one() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE)), 2);
    }

    #[test]
    fn test_part_one_example_two() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO)), 6);
    }

    const EXAMPLE_THREE: &'static str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE_THREE)), 6);
    }

    #[test]
    fn test_lcm_all() {
        assert_eq!(lcm_all(&[2, 3, 4]), 12);
        assert_eq!(lcm_all(&Vec::from_iter(1..10)), 2520);
    }
}
