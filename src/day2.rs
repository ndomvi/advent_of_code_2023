use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<String>;

#[aoc_generator(day2)]
fn parse(input: &str) -> ParsedInput {
    input.lines().map(String::from).collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;

    for (id, l) in input.iter().enumerate() {
        let l = l.split(':').nth(1).expect("Invalid string.");
        if l.split(';').all(|set| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            for cubes in set.split(',') {
                let mut s = cubes.split_ascii_whitespace();
                let num = s.next().unwrap().parse::<i32>().unwrap();
                match s.next().unwrap() {
                    "red" => r += num,
                    "green" => g += num,
                    "blue" => b += num,
                    _ => unreachable!(),
                }
                if r > 12 || g > 13 || b > 14 {
                    return false;
                }
            }

            true
        }) {
            res += id + 1;
        }
    }

    Ok(res as i64)
}

#[aoc(day2, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut power = 0;

    for l in input {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        let l = l.split(':').nth(1).expect("Invalid string.");
        for set in l.split(';') {
            for cubes in set.split(',') {
                let mut s = cubes.split_ascii_whitespace();
                let num = s.next().unwrap().parse::<i32>().unwrap();
                match s.next().unwrap() {
                    "red" => max_r = max_r.max(num),
                    "green" => max_g = max_g.max(num),
                    "blue" => max_b = max_b.max(num),
                    _ => unreachable!(),
                };
            }
        }

        power += max_r * max_g * max_b;
    }

    Ok(power as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 2286);
    }
}
