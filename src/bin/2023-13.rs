type Pattern = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn part_one(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| {
            if let Some(&line) = vertical(p).first() {
                return line;
            } else if let Some(&line) = horizontal(p).first() {
                return line * 100;
            }
            unreachable!()
        })
        .sum()
}

fn part_two(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| {
            let mut q = p.clone();
            let ov = vertical(p);
            let oh = horizontal(p);
            for i in 0..p.len() {
                for j in 0..p[0].len() {
                    q[i][j] = if q[i][j] == '.' { '#' } else { '.' };
                    let v = vertical(&q);
                    let h = horizontal(&q);
                    q[i][j] = if q[i][j] == '.' { '#' } else { '.' };

                    let a = v.iter().find(|&x| !ov.contains(x));
                    let b = h.iter().find(|&x| !oh.contains(x));

                    match (a, b) {
                        (Some(&x), None) => return x,
                        (None, Some(&y)) => return y * 100,
                        (Some(_), Some(_)) => unreachable!(),
                        _ => (),
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

fn vertical(pattern: &Pattern) -> Vec<usize> {
    let mut result = Vec::new();
    let width = pattern[0].len();
    for i in 0..width - 1 {
        let window = (i + 1).min(width - i - 1);
        if (0..window).all(|j| {
            let a = pattern.iter().map(|p| p[i - j]);
            let b = pattern.iter().map(|p| p[i + j + 1]);
            a.eq(b)
        }) {
            result.push(i + 1);
        }
    }
    result
}

fn horizontal(pattern: &Pattern) -> Vec<usize> {
    let mut result = Vec::new();
    let height = pattern.len();
    for i in 0..height - 1 {
        let window = (i + 1).min(height - i - 1);
        if (0..window).all(|j| pattern[i - j] == pattern[i + j + 1]) {
            result.push(i + 1);
        }
    }
    result
}

fn main() {
    let patterns = parse(include_str!("../input/2023-13.txt"));

    println!("Part one: {}", part_one(&patterns));
    println!("Part two: {}", part_two(&patterns));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_vertical() {
        assert_eq!(vertical(&parse(EXAMPLE)[0]), vec![5]);
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(horizontal(&parse(EXAMPLE)[1]), vec![4]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 405);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 400);
    }
}
