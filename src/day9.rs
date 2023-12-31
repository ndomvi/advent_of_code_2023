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

fn extrapolate(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        history.last().unwrap() + extrapolate(&diffs)
    }
}

#[aoc(day9, part1)]
fn part1(input: &ParsedInput) -> i64 {
    input.iter().map(|h| extrapolate(h)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &ParsedInput) -> i64 {
    part1(
        &input
            .iter()
            .map(|h| h.iter().copied().rev().collect::<Vec<_>>())
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
        assert_eq!(part1(&parse(TESTCASE)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 2);
    }
}
