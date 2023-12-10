use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type ParsedInput = Vec<(i64, i64)>;

#[aoc_generator(day6, part1)]
fn parse(input: &str) -> ParsedInput {
    let mut l = input.lines();

    let re = Regex::new(r"([0-9])+").unwrap();
    let times = re
        .find_iter(l.next().unwrap())
        .map(|m| m.as_str().parse::<i64>().unwrap());
    let distances = re
        .find_iter(l.next().unwrap())
        .map(|m| m.as_str().parse::<i64>().unwrap());

    times.zip(distances).collect()
}

#[aoc(day6, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut res = 1;

    for (time, distance) in input {
        let mut variants = 0;
        for i in 0..*time {
            if i * (time - i) > *distance {
                variants += 1;
            }
        }

        if variants > 0 {
            res *= variants;
        }
    }

    res
}

// TODO: finish
// #[aoc(day6, part1, quadratic)]
// fn part1_quadratic(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
//     let mut res = 1;

//     for (time, distance) in input {
//         let time = *time as f64;
//         let distance = *distance as f64;

//         let d_sqrt = (time.powi(2) - 4.0 * distance - 1.0).sqrt();
//         let variants = (((time + d_sqrt).round() as i64 / 2) - ((time - d_sqrt).round() as i64 / 2)) as i64;
//         println!(
//             "{}/{} {} {d_sqrt} == {}",
//             time,
//             distance,
//             time.powi(2) - 4.0 * distance,
//             variants
//         );

//         if variants > 0 {
//             res *= variants;
//         }
//     }

//     Ok(res)
// }

#[aoc_generator(day6, part2)]
fn parse_2(input: &str) -> (i64, i64) {
    let mut l = input.lines();

    let re = Regex::new(r"([0-9])+").unwrap();
    // Feels a bit too complex
    let times = re
        .find_iter(l.next().unwrap())
        .flat_map(|m| m.as_str().chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distances = re
        .find_iter(l.next().unwrap())
        .flat_map(|m| m.as_str().chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    (times, distances)
}

#[aoc(day6, part2)]
fn part2((time, distance): &(i64, i64)) -> i64 {
    let mut variants = 0;
    for i in 0..*time {
        if i * (time - i) > *distance {
            variants += 1;
        }
    }

    variants
}

#[aoc(day6, part2, quadratic)]
fn part2_quadratic((time, distance): &(i64, i64)) -> i64 {
    // The solutions are defined by a parabola
    // -x^2+time*x>distance
    // -x^2+time*x-distance>0
    // d = b^2-4ac = time^2 - 4*distance
    // x = (-b (+-) d)/2a
    let time = *time as f64;
    let distance = *distance as f64;

    let d_sqrt = (time.powi(2) - 4.0 * distance).sqrt();
    (((-time - d_sqrt) / -2.0) - ((-time + d_sqrt) / -2.0)) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"Time:      7  15   30
    Distance:  9  40  200"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 288);
    }

    // #[test]
    // fn part1_quadratic_example() {
    //     assert_eq!(part1_quadratic(&parse(TESTCASE)).unwrap(), 288);
    // }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_2(TESTCASE)), 71503);
    }

    #[test]
    fn part2_quadratic_example() {
        assert_eq!(part2_quadratic(&parse_2(TESTCASE)), 71503);
    }
}
