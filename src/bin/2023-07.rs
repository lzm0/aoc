use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Hand = [Card; 5];

fn hand_type(hand: &Hand) -> HandType {
    let count = hand.iter().fold(HashMap::new(), |mut map, &card| {
        *map.entry(card).or_insert(0) += 1;
        map
    });

    let joker_count = *count.get(&Card::Joker).unwrap_or(&0);

    let mut frequencies = count
        .clone()
        .drain()
        .filter(|&(k, _)| k != Card::Joker)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    if frequencies.len() == 0 {
        return HandType::FiveOfAKind;
    }

    frequencies.sort();
    *frequencies.last_mut().unwrap() += joker_count;

    match frequencies.as_slice() {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 4] => HandType::FourOfAKind,
        [5] => HandType::FiveOfAKind,
        _ => unreachable!(),
    }
}

fn parse(input: &str, joker: bool) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let hand = left
                .chars()
                .map(|c| match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' if joker => Card::Joker,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let bid = right.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn solve(game: &Vec<(Hand, u64)>) -> u64 {
    let mut sorted: Vec<([Card; 5], u64)> = game.clone();
    sorted.sort_by_key(|&(hand, _)| (hand_type(&hand), hand));
    sorted
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .sum()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-07.txt").unwrap();

    println!("Part one: {}", solve(&parse(&input, false)));
    println!("Part two: {}", solve(&parse(&input, true)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &'static str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(solve(&parse(EXAMPLE, false)), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve(&parse(EXAMPLE, true)), 5905);
    }

    #[test]
    fn test_hand_type() {
        // AAAAA: Five of a kind
        assert_eq!(
            hand_type(&[Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
            HandType::FiveOfAKind
        );

        // AA8AA: Four of a kind
        assert_eq!(
            hand_type(&[Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace]),
            HandType::FourOfAKind
        );

        // 23332: Full house
        assert_eq!(
            hand_type(&[Card::Two, Card::Three, Card::Three, Card::Three, Card::Two]),
            HandType::FullHouse
        );

        // TTT98: Three of a kind
        assert_eq!(
            hand_type(&[Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight]),
            HandType::ThreeOfAKind
        );

        // 23432: Two pair
        assert_eq!(
            hand_type(&[Card::Two, Card::Three, Card::Four, Card::Three, Card::Two]),
            HandType::TwoPair
        );

        // A23A4: One pair
        assert_eq!(
            hand_type(&[Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four]),
            HandType::OnePair
        );

        // 23456: High card
        assert_eq!(
            hand_type(&[Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]),
            HandType::HighCard
        );
    }
}
