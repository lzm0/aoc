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

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
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
    find_loop(sketch).len() / 2
}

fn find_loop(sketch: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let m = sketch.len();
    let n = sketch[0].len();

    let start = find_start(sketch);
    let mut stack = vec![(start, Vec::new())];
    let mut visited = HashSet::new();

    while stack.len() > 0 {
        let ((x, y), path) = stack.pop().unwrap();
        let current = sketch[x][y];
        println!(
            "Popping {:?} at {:?} at depth {:?}",
            current,
            (x, y),
            path.len()
        );

        if current == Tile::Start && path.len() > 2 {
            return path;
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
            let (dx, dy) = direction.to_offset();
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize {
                continue;
            }
            if visited.contains(&(nx as usize, ny as usize)) && start != (nx as usize, ny as usize)
            {
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
                path.len() + 1
            );
            let mut new_path = path.clone();
            new_path.push((nx as usize, ny as usize));
            stack.push(((nx as usize, ny as usize), new_path));
        }
    }
    unreachable!()
}

fn part_two(sketch: &Vec<Vec<Tile>>) -> usize {
    let m = sketch.len();
    let n = sketch[0].len();
    let mut scaled = vec![vec![Tile::Ground; n * 3]; m * 3];
    let the_loop = find_loop(sketch);

    for &(x, y) in the_loop.iter() {
        let current = sketch[x][y];
        scaled[x * 3 + 1][y * 3 + 1] = current;
        let directions = match current {
            Tile::Pipe(directions) => directions.to_vec(),
            Tile::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            _ => vec![],
        };
        for direction in directions {
            let (dx, dy) = direction.to_offset();
            let (nx, ny) = (
                (x as isize * 3 + 1 + dx) as usize,
                (y as isize * 3 + 1 + dy) as usize,
            );
            scaled[nx][ny] = current;
        }
    }

    let mut stack = vec![(0, 0)];
    let mut visited = HashSet::new();
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let (dx, dy) = direction.to_offset();
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= 3 * m as isize || ny < 0 || ny >= 3 * n as isize {
                continue;
            }
            if visited.contains(&(nx as usize, ny as usize)) {
                continue;
            }
            let next = scaled[nx as usize][ny as usize];
            if next != Tile::Ground {
                continue;
            }

            stack.push((nx as usize, ny as usize));
        }
    }
    visualize(&scaled);
    dbg!(the_loop.len());
    let mut enclosed = 0;
    for (x, row) in sketch.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if !the_loop.contains(&(x, y)) && !visited.contains(&(3 * x + 1, 3 * y + 1)) {
                enclosed += 1;
            }
        }
    }
    enclosed
}

fn visualize(sketch: &Vec<Vec<Tile>>) {
    for row in sketch.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Pipe([Direction::North, Direction::South]) => print!("|"),
                Tile::Pipe([Direction::East, Direction::West]) => print!("-"),
                Tile::Pipe([Direction::North, Direction::East]) => print!("L"),
                Tile::Pipe([Direction::North, Direction::West]) => print!("J"),
                Tile::Pipe([Direction::South, Direction::West]) => print!("7"),
                Tile::Pipe([Direction::South, Direction::East]) => print!("F"),
                Tile::Pipe(_) => unreachable!(),
                Tile::Ground => print!("."),
                Tile::Start => print!("S"),
            }
        }
        println!();
    }
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
        assert_eq!(part_two(&parse(EXAMPLE_ONE)), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(part_two(&parse(EXAMPLE_TWO)), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(part_two(&parse(EXAMPLE_THREE)), 8);
    }

    #[test]
    fn example_four() {
        assert_eq!(part_two(&parse(EXAMPLE_FOUR)), 10);
    }
}

#[cfg(test)]
mod test_find_loop {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &'static str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    #[test]
    fn example() {
        assert_eq!(
            find_loop(&parse(EXAMPLE)),
            vec![
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
                (1, 1)
            ]
        );
    }
}
