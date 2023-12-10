use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<Vec<char>>;

#[aoc_generator(day3)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

const NEIGHBORS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn check_pos(i: usize, j: usize, lines: &Vec<Vec<char>>) -> bool {
    for (di, dj) in NEIGHBORS {
        let ai = i as i32 + di;
        let aj = j as i32 + dj;
        if ai >= 0 && ai < lines.len() as i32 && aj >= 0 && aj < lines[ai as usize].len() as i32 {
            let c = lines[ai as usize][aj as usize];
            if c != '.' && !c.is_ascii_digit() {
                return true;
            }
        }
    }
    false
}

#[aoc(day3, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;
    for (i, line) in input.iter().enumerate() {
        let mut valid = false;
        let mut cur = String::new();
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                cur.push(*char);
                if !valid {
                    valid = check_pos(i, j, input);
                }
            } else {
                if !cur.is_empty() && valid {
                    res += cur.parse::<i32>()?;
                }
                cur.clear();
                valid = false;
            }
        }

        if !cur.is_empty() && valid {
            res += cur.parse::<i32>()?;
        }
    }

    Ok(res as i64)
}

#[aoc(day3, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        let mut adj_gears = HashSet::new();
        let mut cur = String::new();
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                cur.push(*char);
                if let Some((ai, aj)) = check_gear(i, j, input) {
                    adj_gears.insert((ai, aj));
                }
            } else {
                if !cur.is_empty() {
                    let num = cur.parse::<i32>()?;
                    for g in &adj_gears {
                        gears
                            .entry(*g)
                            .and_modify(|i| i.push(num))
                            .or_insert_with(|| vec![num]);
                    }
                }
                cur.clear();
                adj_gears.clear();
            }
        }

        if !cur.is_empty() {
            let num = cur.parse::<i32>()?;
            for g in &adj_gears {
                gears
                    .entry(*g)
                    .and_modify(|i| i.push(num))
                    .or_insert_with(|| vec![num]);
            }
        }
    }

    let res = gears.iter().fold(0, |acc, (_, g)| match g.len() {
        2 => g.iter().product::<i32>() + acc,
        _ => acc,
    });
    Ok(res as i64)
}

fn check_gear(i: usize, j: usize, lines: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (di, dj) in NEIGHBORS {
        let ai = i as i32 + di;
        let aj = j as i32 + dj;
        if ai >= 0 && ai < lines.len() as i32 && aj >= 0 && aj < lines[ai as usize].len() as i32 {
            let c = lines[ai as usize][aj as usize];
            if c != '.' && !c.is_ascii_digit() {
                return Some((ai as usize, aj as usize));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 467835);
    }
}
