use std::{collections::HashMap, error::Error};

use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;
use regex::Regex;

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

type ParsedInput = Network;

#[aoc_generator(day8)]
fn parse(input: &str) -> ParsedInput {
    let mut l = input.lines();
    let instructions = l.next().unwrap().chars().collect::<Vec<_>>();

    let re = Regex::new(r"([a-zA-Z0-9]+)").unwrap();
    let nodes = l
        .skip(1)
        .map(|l| {
            let mut i = re.find_iter(l);
            let start = i.next().unwrap().as_str().to_string();
            let l = i.next().unwrap().as_str().to_string();
            let r = i.next().unwrap().as_str().to_string();
            (start, (l, r))
        })
        .collect::<HashMap<_, _>>();

    Network {
        instructions,
        nodes,
    }
}

#[aoc(day8, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut cur = "AAA".to_string();
    let mut counter = 0;
    let mut i = input.instructions.iter().cycle();
    while cur != "ZZZ" {
        let cur_map = input.nodes.get(&cur).expect("Unknown node.");
        match i.next().unwrap() {
            'L' => cur = cur_map.0.clone(),
            'R' => cur = cur_map.1.clone(),
            _ => unreachable!(),
        }

        counter += 1;
    }

    Ok(counter)
}

#[aoc(day8, part2)]
// This one is a minor bruh moment
// I've kind of cheated here, because I've seen that someone else is using LCM to solve it.
// It makes sense to use it to find when the cycles "intersect", but I feel like you would first need to find the cycle
// which the function doesn't do.
// Maybe it's a property of the inputs, idk.
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let starts = input
        .nodes
        .iter()
        .filter_map(|n| if n.0.ends_with('A') { Some(n.0) } else { None })
        // .map(|s| {
        //     let mut cur = s;
        //     let mut i = input.instructions.iter().cycle();
        //     while !cur.ends_with('Z') {
        //         let cur_map = input.nodes.get(cur).expect("Unknown node.");
        //         match i.next().unwrap() {
        //             'L' => cur = &cur_map.0,
        //             'R' => cur = &cur_map.1,
        //             _ => unreachable!(),
        //         }
        //     }
        //     cur
        // })
        .collect::<Vec<_>>();

    let mut counters = vec![];
    for s in starts {
        let mut cur = s;
        let mut counter = 0;
        let mut i = input.instructions.iter().cycle();
        while !cur.ends_with('Z') {
            let cur_map = input.nodes.get(cur).expect("Unknown node.");
            match i.next().unwrap() {
                'L' => cur = &cur_map.0,
                'R' => cur = &cur_map.1,
                _ => unreachable!(),
            }

            counter += 1
        }
        counters.push(counter);
    }

    Ok(counters.iter().copied().reduce(|a, b| a.lcm(&b)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    const TESTCASE2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 2);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(TESTCASE2)).unwrap(), 6);
    }

    const TESTCASE_P2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE_P2)).unwrap(), 6);
    }
}
