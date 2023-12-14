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
            if let Some(line) = vertical(p) {
                return line;
            } else if let Some(line) = horizontal(p) {
                return line * 100;
            }
            unreachable!()
        })
        .sum()
}

fn vertical(pattern: &Pattern) -> Option<usize> {
    let width = pattern[0].len();
    for i in 0..width - 1 {
        let window = (i + 1).min(width - i - 1);
        if (0..window).all(|j| {
            let a = pattern.iter().map(|p| p[i - j]);
            let b = pattern.iter().map(|p| p[i + j + 1]);
            a.eq(b)
        }) {
            return Some(i + 1);
        }
    }
    None
}

fn horizontal(pattern: &Pattern) -> Option<usize> {
    let height = pattern.len();
    for i in 0..height - 1 {
        let window = (i + 1).min(height - i - 1);
        if (0..window).all(|j| pattern[i - j] == pattern[i + j + 1]) {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    let patterns = parse(include_str!("../input/2023-13.txt"));

    println!("Part one: {}", part_one(&patterns));
    // println!("Part two: {}", part_two(&records));
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
    fn test_part_one() {
        let patterns = parse(EXAMPLE);
        assert_eq!(part_one(&patterns), 405);
    }

    #[test]
    fn test_vertical() {
        assert_eq!(vertical(&parse(EXAMPLE)[0]), Some(5));
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(horizontal(&parse(EXAMPLE)[1]), Some(4));
    }

    // #[test]
    // fn test_part_two() {
    //     let patterns = parse(EXAMPLE);
    //     assert_eq!(part_two(&patterns), 525152);
    // }
}
