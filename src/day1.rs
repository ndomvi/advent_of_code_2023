use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(file: &str) -> Vec<String> {
    file.lines().map(String::from).collect::<Vec<_>>()
}

#[aoc(day1, part1)]
fn part1(lines: &[String]) -> i64 {
    let mut result = 0;
    for l in lines {
        let digits = l.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();

        result += (digits
            .first()
            .expect("A line has no digits (must be >= 1).")
            .to_string()
            + &digits
                .last()
                .expect("A char just disappeared from an array")
                .to_string())
            .parse::<i64>()
            .unwrap();
    }

    result
}

#[aoc(day1, part2)]
fn part2(lines: &[String]) -> i64 {
    let lines = lines
        .iter()
        .map(|l| {
            let mut l = l.replace("one", "one1one");
            l = l.replace("two", "two2two");
            l = l.replace("three", "three3three");
            l = l.replace("four", "four4four");
            l = l.replace("five", "five5five");
            l = l.replace("six", "six6six");
            l = l.replace("seven", "seven7seven");
            l = l.replace("eight", "eight8eight");
            l = l.replace("nine", "nine9nine");
            l
        })
        .collect::<Vec<_>>();
    part1(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            )),
            281
        );
    }
}
