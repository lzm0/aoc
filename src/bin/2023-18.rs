type Dig = (u8, i64, u32);

fn parse(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|s| {
            let mut s = s.split(&[' ', '(', '#', ')']).filter(|x| !x.is_empty());
            (
                s.next().unwrap().as_bytes()[0],
                s.next().unwrap().parse().unwrap(),
                u32::from_str_radix(s.next().unwrap(), 16).unwrap(),
            )
        })
        .collect()
}

fn solve(commands: &Vec<Dig>) -> i64 {
    let (mut x, mut y, mut t) = (0, 0, 2);
    let mut edges = commands
        .iter()
        .map(|(dir, len, ..)| {
            match dir {
                b'U' => y -= len,
                b'D' => y += len,
                b'L' => x -= len,
                _ => x += len,
            };
            t += len;
            (x, y)
        })
        .collect::<Vec<_>>();
    edges.push(edges[0]);
    (edges
        .windows(2)
        .map(|i| i[0].0 * i[1].1 - i[0].1 * i[1].0)
        .sum::<i64>()
        + t)
        / 2
}

fn part_one(commands: &Vec<Dig>) -> i64 {
    solve(commands)
}

fn part_two(commands: &Vec<Dig>) -> i64 {
    solve(
        &commands
            .iter()
            .map(|&(_, _, color)| (b"RDLU"[color as usize % 16], color as i64 / 16, 0))
            .collect::<Vec<_>>(),
    )
}

fn main() {
    let commands = parse(include_str!("../input/2023-18.txt"));

    println!("Part one: {}", part_one(&commands));
    println!("Part two: {}", part_two(&commands));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 62);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 952408144115);
    }
}
