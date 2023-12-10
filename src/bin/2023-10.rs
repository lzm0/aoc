use std::{collections::HashSet, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Pipe([Direction; 2]),
    Ground,
    Start,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::Pipe([Direction::North, Direction::South]),
                    '-' => Tile::Pipe([Direction::East, Direction::West]),
                    'L' => Tile::Pipe([Direction::North, Direction::East]),
                    'J' => Tile::Pipe([Direction::North, Direction::West]),
                    '7' => Tile::Pipe([Direction::South, Direction::West]),
                    'F' => Tile::Pipe([Direction::South, Direction::East]),
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_one(sketch: &Vec<Vec<Tile>>) -> usize {
    let m = sketch.len();
    let n = sketch[0].len();

    let start = find_start(sketch);
    let mut stack = vec![(start, 0)];
    let mut visited = HashSet::new();

    while stack.len() > 0 {
        let ((x, y), depth) = stack.pop().unwrap();
        println!("Popping {:?} at {:?}", sketch[x][y], (x, y));

        if (x, y) == start && depth > 0 {
            return depth / 2;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        let directions = match sketch[x][y] {
            Tile::Pipe(d) => d.to_vec(),
            Tile::Ground => unreachable!(),
            Tile::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        };
        for direction in directions {
            println!("Going {:?} ", direction);
            let (dx, dy) = match direction {
                Direction::North => (-1, 0),
                Direction::East => (0, 1),
                Direction::South => (1, 0),
                Direction::West => (0, -1),
            };
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                continue;
            }
            let next = sketch[nx as usize][ny as usize];
            println!("Found {:?}", next);
            match next {
                Tile::Pipe(next_directions) => {
                    if !next_directions.contains(&direction.opposite()) {
                        continue;
                    }
                }
                Tile::Ground => continue,
                Tile::Start => (),
            }

            println!("Pushing {:?} at {:?} ", next, (nx, ny));
            stack.push(((nx as usize, ny as usize), depth + 1));
        }

        visited.insert((x, y));
    }
    unreachable!()
}

fn find_start(sketch: &Vec<Vec<Tile>>) -> (usize, usize) {
    for (x, row) in sketch.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-10.txt").unwrap();

    println!("Part one: {}", part_one(&parse(&input)));
    // println!("Part two: {}", part_two(&parse(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_ONE: &'static str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    const EXAMPLE_ONE_SIMPLE: &'static str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const EXAMPLE_TWO: &'static str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    const EXAMPLE_TWO_SIMPLE: &'static str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part_one_example_one_simple() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE_SIMPLE)), 4);
    }

    #[test]
    fn test_part_one_example_one() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE)), 4);
    }

    #[test]
    fn test_part_one_example_two_simple() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO_SIMPLE)), 8);
    }

    #[test]
    fn test_part_one_example_two() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO)), 8);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(&parse(EXAMPLE)), 8);
    // }
}
