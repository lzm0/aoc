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
        let current = sketch[x][y];
        println!("Popping {:?} at {:?} at depth {:?}", current, (x, y), depth);

        if current == Tile::Start && depth > 2 {
            return depth / 2;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let directions = match current {
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
            if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize {
                continue;
            }
            let next = sketch[nx as usize][ny as usize];
            match next {
                Tile::Pipe(next_directions) => {
                    if !next_directions.contains(&direction.opposite()) {
                        continue;
                    }
                }
                Tile::Ground => continue,
                Tile::Start => (),
            }

            println!(
                "Pushing {:?} at {:?} at depth {:?}",
                next,
                (nx, ny),
                depth + 1
            );
            stack.push(((nx as usize, ny as usize), depth + 1));
        }
    }
    unreachable!()
}

fn part_two(sketch: &Vec<Vec<Tile>>) -> usize {
    0
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
    println!("Part two: {}", part_two(&parse(&input)));
}

#[cfg(test)]
mod test_part_one {
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
    fn example_one_simple() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE_SIMPLE)), 4);
    }

    #[test]
    fn example_one() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE)), 4);
    }

    #[test]
    fn example_two_simple() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO_SIMPLE)), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO)), 8);
    }
}

#[cfg(test)]
mod test_part_two {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_ONE: &'static str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const EXAMPLE_TWO: &'static str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    const EXAMPLE_THREE: &'static str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    const EXAMPLE_FOUR: &'static str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn example_one() {
        assert_eq!(part_one(&parse(EXAMPLE_ONE)), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(part_one(&parse(EXAMPLE_TWO)), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(part_one(&parse(EXAMPLE_THREE)), 8);
    }

    #[test]
    fn example_four() {
        assert_eq!(part_one(&parse(EXAMPLE_FOUR)), 10);
    }
}
