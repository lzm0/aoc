use std::fs;

fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            (
                left.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
                right
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn count_matches(winning_numbers: &Vec<u8>, numbers_you_have: &Vec<u8>) -> usize {
    winning_numbers
        .iter()
        .filter(|&winning_number| numbers_you_have.contains(winning_number))
        .count()
}

fn part_one(cards: &Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    cards
        .iter()
        .map(|(winning_numbers, numbers_you_have)| {
            let matches = count_matches(winning_numbers, numbers_you_have) as u32;
            if matches == 0 {
                0
            } else {
                (2 as u32).pow(matches - 1)
            }
        })
        .sum()
}

fn part_two(cards: &Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let mut stack: Vec<(usize, &(Vec<u8>, Vec<u8>))> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| (i + 1, card))
        .collect();
    let mut total_cards = 0;
    while let Some((card_number, scratchcards)) = stack.pop() {
        total_cards += 1;
        let (winning_numbers, numbers_you_have) = scratchcards;
        let matches = count_matches(winning_numbers, numbers_you_have);
        for i in 0..matches {
            stack.push((card_number + 1 + i, &cards[card_number + i]));
        }
    }
    total_cards
}

fn main() {
    let input = fs::read_to_string("src/input/2023-04.txt").unwrap();
    let cards = parse(&input);

    println!("Part one: {}", part_one(&cards));
    println!("Part two: {}", part_two(&cards));
}

#[test]
fn test_part_one() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let cards = parse(&input);
    assert_eq!(part_one(&cards), 13);
}

#[test]
fn test_part_two() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let cards = parse(&input);
    assert_eq!(part_two(&cards), 30);
}
