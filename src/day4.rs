use std::{collections::HashSet, error::Error};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect::<Vec<_>>()
}

#[aoc(day4, part1)]
fn part1(lines: &[String]) -> i64 {
    let mut res = 0;

    for line in lines {
        let line = line.split(':').nth(1).unwrap();
        let scratched: HashSet<&str> = line
            .split('|')
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect();
        let winning: HashSet<&str> = line
            .split('|')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .collect();

        let common = scratched.intersection(&winning).count();
        if common > 0 {
            res += 2i32.pow(common as u32 - 1);
        }
    }

    res as i64
}

#[aoc(day4, part2)]
fn part2(lines: &[String]) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;

    let mut copies = vec![1; lines.len()];
    for (id, line) in lines.iter().enumerate() {
        let line = line.split(':').nth(1).ok_or("Invalid line.")?;

        let mut split = line.split('|');
        let scratched: HashSet<&str> = split
            .next()
            .ok_or("Invalid line.")?
            .split_ascii_whitespace()
            .collect();
        let winning: HashSet<&str> = split
            .next()
            .ok_or("Invalid line.")?
            .split_ascii_whitespace()
            .collect();
        res += copies[id];
        for i in 1..=scratched.intersection(&winning).count() {
            if id + i < copies.len() {
                copies[id + i] += copies[id];
            }
        }
    }

    Ok(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            13
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ))
            .unwrap(),
            30
        );
    }
}
