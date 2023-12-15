use std::str::from_utf8;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::bytes::Regex;
use smallvec::SmallVec;

type ParsedInput = Vec<SmallVec<[u8; 16]>>;

#[aoc_generator(day15)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.chars().map(|c| c as u8).collect::<_>())
        .collect::<_>()
}

fn hash(input: &[u8]) -> u64 {
    let mut cur = 0;
    for c in input {
        cur += *c as u64;
        cur *= 17;
        cur %= 256;
    }
    cur
}

#[aoc(day15, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut res = 0;

    for step in input {
        res += hash(step);
    }

    res as i64
}

#[aoc(day15, part2)]
fn part2(input: &ParsedInput) -> usize {
    let mut res = 0;

    let mut boxes: Vec<Vec<(&[u8], u8)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let re = Regex::new("([a-zA-Z]+)([=-])([0-9]+)?").unwrap();
    for step in input {
        let capture = re.captures(step).unwrap();
        match (
            capture.get(1).map(|m| m.as_bytes()),
            capture.get(2).map(|m| m.as_bytes()[0]),
            capture
                .get(3)
                .map(|m| from_utf8(m.as_bytes()).unwrap().parse::<u8>().unwrap()),
        ) {
            (Some(lbl), Some(b'-'), None) => {
                let bx = boxes.get_mut(hash(lbl) as usize).unwrap();
                for i in 0..bx.len() {
                    if bx[i].0 == lbl {
                        bx.remove(i);
                        break;
                    }
                }
            }
            (Some(lbl), Some(b'='), Some(val)) => {
                let bx = boxes.get_mut(hash(lbl) as usize).unwrap();
                let mut found = false;
                for i in 0..bx.len() {
                    if bx[i].0 == lbl {
                        bx.get_mut(i).unwrap().1 = val;
                        found = true;
                        break;
                    }
                }

                if !found {
                    bx.push((lbl, val));
                }
            }
            _ => unreachable!(),
        }
    }

    for (bx_idx, bx) in boxes.iter().enumerate() {
        for (lens_idx, (_, val)) in bx.iter().enumerate() {
            res += (bx_idx + 1) * (lens_idx + 1) * *val as usize;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 145);
    }
}
