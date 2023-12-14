enum Direction {
    North,
    West,
    South,
    East,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn tilt(platform: &Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    todo!()
}

fn part_one(platform: &Vec<Vec<char>>) -> usize {
    let rows = platform.len();
    let cols = platform[0].len();
    let mut load = 0;
    for col in 0..cols {
        let mut slot = 0;
        for row in 0..rows {
            match platform[row][col] {
                'O' => {
                    load += rows - slot;
                    slot += 1;
                }
                '#' => slot = row + 1,
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
    load
}

fn part_two(platform: &Vec<Vec<char>>) -> usize {
    todo!()
}

fn main() {
    let platform = parse(include_str!("../input/2023-14.txt"));

    println!("Part one: {}", part_one(&platform));
    println!("Part two: {}", part_two(&platform));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 64);
    }
}
