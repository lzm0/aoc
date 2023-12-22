use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, r) in map.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if c == 'S' {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn part_one(map: &Vec<Vec<char>>, max_steps: u64) -> u64 {
    let (x, y) = start(map);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(x, y, 0)]);
    while let Some((x, y, steps)) = queue.pop_front() {
        if steps > max_steps {
            break;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if x > 0 && map[y][x - 1] != '#' {
            queue.push_back((x - 1, y, steps + 1));
        }
        if y > 0 && map[y - 1][x] != '#' {
            queue.push_back((x, y - 1, steps + 1));
        }
        if x < map[y].len() - 1 && map[y][x + 1] != '#' {
            queue.push_back((x + 1, y, steps + 1));
        }
        if y < map.len() - 1 && map[y + 1][x] != '#' {
            queue.push_back((x, y + 1, steps + 1));
        }
    }

    visited
        .iter()
        .filter(|&&(x2, y2)| ((x2 as i64 - x as i64).abs() + (y2 as i64 - y as i64).abs()) % 2 == 0)
        .count() as u64
}

fn part_two(map: &Vec<Vec<char>>) -> u64 {
    0
}

fn main() {
    let map = parse(include_str!("../input/2023-21.txt"));

    println!("Part one: {}", part_one(&map, 64));
    println!("Part two: {}", part_two(&map));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE), 6), 16);
    }
}
