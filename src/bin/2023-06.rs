use std::fs;

/* Note
 *
 * This solution uses the quadratic formula.
 * Using f32 instead of f64 will produce incorrect results. */

fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines().into_iter();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    times.zip(distances).collect()
}

fn part_one(races: &Vec<(u64, u64)>) -> u64 {
    races
        .iter()
        .map(|&(time, distance)| {
            let t = time as f64;
            let d = distance as f64;
            let descriminant = (t.powi(2) - 4.0 * d).sqrt();
            let x1;
            let x2;
            if descriminant > 0.0 {
                x1 = (t - descriminant) / 2.0;
                x2 = (t + descriminant) / 2.0;
            } else {
                x1 = (t + descriminant) / 2.0;
                x2 = (t - descriminant) / 2.0;
            }
            x2.ceil() as u64 - x1.floor() as u64 - 1
        })
        .product()
}

fn part_two(races: &Vec<(u64, u64)>) -> u64 {
    let time = races
        .iter()
        .map(|&(t, _)| t.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();
    let distance = races
        .iter()
        .map(|&(_, d)| d.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();
    part_one(&vec![(time, distance)])
}

fn main() {
    let input = fs::read_to_string("src/input/2023-06.txt").unwrap();
    let races = parse(&input);

    println!("Part one: {}", part_one(&races));
    println!("Part two: {}", part_two(&races));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE)), 71503);
    }
}
