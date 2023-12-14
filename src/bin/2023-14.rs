fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn tilt(platform: &mut Vec<Vec<char>>) {
    for col in 0..platform[0].len() {
        let mut slot = 0;
        for row in 0..platform.len() {
            let curr = platform[row][col];
            match curr {
                'O' => {
                    platform[row][col] = platform[slot][col];
                    platform[slot][col] = curr;
                    slot += 1;
                }
                '#' => slot = row + 1,
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
}

fn clockwise(platform: &mut Vec<Vec<char>>) {
    let size = platform.len();
    let mut new = vec![vec!['.'; size]; size];
    for row in 0..size {
        for col in 0..size {
            new[col][size - row - 1] = platform[row][col];
        }
    }
    *platform = new;
}

fn total_load(platform: &Vec<Vec<char>>) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .map(|&c| {
                    if c == 'O' {
                        platform.len() - i
                    } else {
                        0 as usize
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt(platform);
        clockwise(platform);
    }
}

fn part_one(platform: &Vec<Vec<char>>) -> usize {
    let mut platform = platform.clone();
    tilt(&mut platform);
    total_load(&platform)
}

fn part_two(platform: &Vec<Vec<char>>) -> usize {
    let mut platform = platform.clone();
    let mut seen = vec![platform.clone()];

    loop {
        cycle(&mut platform);
        if let Some(i) = seen.iter().position(|x| *x == platform) {
            let cycle_len = seen.len() - i;
            let final_state = &seen[i + (1000000000 - i) % cycle_len];
            return total_load(final_state);
        }
        seen.push(platform.clone());
    }
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
    fn test_tilt() {
        let mut platform = parse(EXAMPLE);
        tilt(&mut platform);
        assert_eq!(
            platform,
            parse(indoc! {"
                OOOO.#.O..
                OO..#....#
                OO..O##..O
                O..#.OO...
                ........#.
                ..#....#.#
                ..O..#.O.O
                ..O.......
                #....###..
                #....#....
                "})
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 64);
    }
}
