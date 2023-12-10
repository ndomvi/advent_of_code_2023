use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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

impl Card {
    fn new(card_char: char) -> Self {
        match card_char.to_ascii_lowercase() {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            't' => Self::Ten,
            'j' => Self::Jack,
            'q' => Self::Queen,
            'k' => Self::King,
            'a' => Self::Ace,
            _ => unreachable!("Invalid card!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Hand {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Game {
    hand: Hand,
    cards: Vec<Card>,
    bet: i64,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| match a.cmp(b) {
                    Ordering::Equal => None,
                    o => Some(o),
                })
                .unwrap(),
            o => o,
        }
    }
}

type ParsedInput = Vec<Game>;
#[aoc_generator(day7, part1)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            let mut l = l.split_ascii_whitespace();
            let cards = l.next().unwrap().chars().map(Card::new).collect::<Vec<_>>();

            let mut card_counts = HashMap::new();
            for c in &cards {
                card_counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
            }

            let mut values = card_counts.values().collect::<Vec<_>>();
            values.sort();
            let hand = match values.len() {
                5 => Hand::High,
                4 => Hand::OnePair,
                3 => {
                    if *values[2] == 2 {
                        Hand::TwoPair
                    } else {
                        Hand::ThreeKind
                    }
                }
                2 => {
                    if *values[1] == 4 {
                        Hand::FourKind
                    } else {
                        Hand::FullHouse
                    }
                }
                1 => Hand::FiveKind,
                _ => unreachable!(),
            };

            Game {
                hand,
                cards,
                bet: l.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut input = input.clone();
    input.sort();
    let mut res = 0;
    for (i, game) in input.iter().enumerate() {
        res += (i + 1) * game.bet as usize;
    }
    res as i64
}

#[aoc_generator(day7, part2)]
fn parse2(input: &str) -> ParsedInput {
    let mut out = parse(input);
    for g in &mut out {
        let mut jokers = 0;
        g.cards.iter_mut().for_each(|c| {
            if *c == Card::Jack {
                *c = Card::Joker;
                jokers += 1;
            }
        });

        while jokers > 0 {
            g.hand = match g.hand {
                Hand::High => Hand::OnePair,
                Hand::OnePair => {
                    if jokers == 2 {
                        Hand::OnePair
                    } else {
                        Hand::ThreeKind
                    }
                }
                Hand::TwoPair => {
                    if jokers == 3 {
                        Hand::TwoPair
                    } else {
                        Hand::FullHouse
                    }
                }
                Hand::ThreeKind => {
                    if jokers == 3 {
                        jokers = 0;
                    }
                    Hand::FourKind
                }
                Hand::FullHouse => {
                    if jokers == 3 {
                        Hand::FullHouse
                    } else {
                        Hand::FourKind
                    }
                }
                Hand::FourKind | Hand::FiveKind => Hand::FiveKind,
            };
            jokers -= 1;
        }
    }

    out
}

#[aoc(day7, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let mut input = input.clone();
    input.sort();
    let mut res = 0;
    for (i, game) in input.iter().enumerate() {
        res += (i + 1) * game.bet as usize;
    }
    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE_R: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"#;
    #[test]
    fn part1_example_r() {
        assert_eq!(part1(&parse(TESTCASE_R)), 6592);
    }

    #[test]
    fn part2_example_r() {
        assert_eq!(part2(&parse2(TESTCASE_R)), 6839);
    }

    const TESTCASE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 6440);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse2(TESTCASE)).unwrap(), 5905);
    // }
}
