#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Empty,
    Galaxy,
}

fn parse(input: &str) -> Vec<Vec<Pixel>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Pixel::Empty,
                    '#' => Pixel::Galaxy,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn solve(image: &Vec<Vec<Pixel>>, expansion: usize) -> usize {
    let row_indices = image
        .iter()
        .scan(0, |state, row| {
            let index = Some(*state);
            if row.contains(&Pixel::Galaxy) {
                *state += 1;
            } else {
                *state += expansion;
            }
            index
        })
        .collect::<Vec<_>>();
    let col_indices = (0..image[0].len())
        .scan(0, |state, i| {
            let index = Some(*state);
            if image.iter().any(|row| row[i] == Pixel::Galaxy) {
                *state += 1;
            } else {
                *state += expansion;
            }
            index
        })
        .collect::<Vec<_>>();
    let galaxies = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &pixel)| pixel == Pixel::Galaxy)
                .map(|(j, _)| (row_indices[i], col_indices[j]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .enumerate()
        .map(|(i, &(x1, y1))| {
            galaxies
                .iter()
                .enumerate()
                .map(|(j, &(x2, y2))| {
                    if i < j {
                        ((x2 as isize - x1 as isize).abs() + (y2 as isize - y1 as isize).abs())
                            as usize
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_one(image: &Vec<Vec<Pixel>>) -> usize {
    solve(image, 2)
}

fn part_two(image: &Vec<Vec<Pixel>>) -> usize {
    solve(image, 1_000_000)
}

fn main() {
    let image = parse(include_str!("../input/2023-11.txt"));

    println!("Part one: {}", part_one(&image));
    println!("Part two: {}", part_two(&image));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 374);
    }

    #[test]
    fn test_solve_10() {
        assert_eq!(solve(&parse(EXAMPLE), 10), 1030);
    }

    #[test]
    fn test_solve_100() {
        assert_eq!(solve(&parse(EXAMPLE), 100), 8410);
    }
}
