use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<Vec<i64>>;

#[aoc_generator(day9)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().expect("Could not parse a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_history(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    history.last().unwrap() + parse_history(&diffs)
}

#[aoc(day9, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;

    for hist in input {
        res += parse_history(hist);
    }

    Ok(res)
}

#[aoc(day9, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    part1(
        &input
            .iter()
            .map(|h| h.iter().cloned().rev().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 2);
    }
}
