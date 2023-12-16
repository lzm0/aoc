use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn interact(&self, tile: char) -> Vec<Direction> {
        match tile {
            '.' => vec![*self],
            '/' => match self {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            '\\' => match self {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            '|' => match self {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
            },
            '-' => match self {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
            },
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn shoot(
    row: usize,
    col: usize,
    direction: Direction,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize, Direction)>,
) {
    if visited.contains(&(row, col, direction)) {
        return;
    }
    visited.insert((row, col, direction));
    direction.interact(grid[row][col]).iter().for_each(|&d| {
        let (r, c) = d.offset();
        let (row, col) = (row as isize + r, col as isize + c);
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return;
        }
        let (row, col) = (row as usize, col as usize);
        shoot(row, col, d, grid, visited);
    });
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    shoot(0, 0, Direction::Right, grid, &mut visited);
    HashSet::<(usize, usize)>::from_iter(visited.iter().map(|&(row, col, _)| (row, col))).len()
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let mut starts = Vec::new();
    for i in 0..grid.len() {
        starts.push((i, 0, Direction::Right));
        starts.push((i, grid[0].len() - 1, Direction::Left));
    }
    for i in 0..grid[0].len() {
        starts.push((0, i, Direction::Down));
        starts.push((grid.len() - 1, i, Direction::Up));
    }
    let mut max = 0;
    for &(row, col, direction) in &starts {
        let mut visited = HashSet::new();
        shoot(row, col, direction, grid, &mut visited);
        max = max.max(
            HashSet::<(usize, usize)>::from_iter(visited.iter().map(|&(row, col, _)| (row, col)))
                .len(),
        );
    }
    max
}

fn main() {
    let grid = parse(include_str!("../input/2023-16.txt"));

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 51);
    }
}
