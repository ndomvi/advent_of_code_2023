use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;
use smallvec::{smallvec, SmallVec};

type ParsedInput = Vec<Vec<u32>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node(i32, i32, SmallVec<[(i8, i8); 8]>);

impl Node {
    fn successors(&self, map: &ParsedInput, len: usize) -> Vec<(Node, u32)> {
        let (sx, sy) = self
            .2
            .iter()
            .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

        let mut res = vec![];
        // East
        if sx < 3 && self.0 + 1 < len as i32 && self.2[2] != (-1, 0) {
            res.push((
                Node(self.0 + 1, self.1, smallvec![self.2[1], self.2[2], (1, 0)]),
                map[self.1 as usize][self.0 as usize + 1],
            ))
        }
        // West
        if sx > -3 && self.0 > 0 && self.2[2] != (1, 0) {
            res.push((
                Node(self.0 - 1, self.1, smallvec![self.2[1], self.2[2], (-1, 0)]),
                map[self.1 as usize][self.0 as usize - 1],
            ))
        }
        // North
        if sy > -3 && self.1 > 0 && self.2[2] != (0, 1) {
            res.push((
                Node(self.0, self.1 - 1, smallvec![self.2[1], self.2[2], (0, -1)]),
                map[self.1 as usize - 1][self.0 as usize],
            ))
        }
        // South
        if sy < 3 && self.1 + 1 < len as i32 && self.2[2] != (0, -1) {
            res.push((
                Node(self.0, self.1 + 1, smallvec![self.2[1], self.2[2], (0, 1)]),
                map[self.1 as usize + 1][self.0 as usize],
            ))
        }
        res
    }

    fn heat_loss_to(&self, x: i32, y: i32, map: &ParsedInput) -> u32 {
        assert!(self.0 == x || self.1 == y);
        let mut res = 0;
        for y in self.1.min(y)..=self.1.max(y) {
            for x in self.0.min(x)..=self.0.max(x) {
                res += map[y as usize][x as usize];
            }
        }
        res - map[self.1 as usize][self.0 as usize]
    }

    // Minor(major) bruh moment, idk if i want to refactor it though
    fn successors_p2(&self, map: &ParsedInput) -> Vec<(Node, u32)> {
        let (sx, sy) = self
            .2
            .iter()
            .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

        let mut res = vec![];
        let hist_o: SmallVec<[(i8, i8); 8]> = SmallVec::from_slice(&self.2[1..]);
        // East
        let mut hist = hist_o.clone();
        if *self.2.last().unwrap() == (1, 0) && sx < 7 && self.0 + 1 < map[0].len() as i32 {
            hist.push((1, 0));
            res.push((
                Node(self.0 + 1, self.1, hist),
                map[self.1 as usize][self.0 as usize + 1],
            ));
        } else if sx < 7 && self.0 + 4 < map[0].len() as i32 && *self.2.last().unwrap() != (-1, 0) {
            hist.push((1, 0));
            res.push((
                Node(self.0 + 4, self.1, hist),
                self.heat_loss_to(self.0 + 4, self.1, map),
            ));
        }
        // West
        let mut hist = hist_o.clone();
        if *self.2.last().unwrap() == (-1, 0) && sx > -7 && self.0 > 0 {
            hist.push((-1, 0));
            res.push((
                Node(self.0 - 1, self.1, hist),
                map[self.1 as usize][self.0 as usize - 1],
            ));
        } else if sx > -7 && self.0 - 4 >= 0 && *self.2.last().unwrap() != (1, 0) {
            hist.push((-1, 0));
            res.push((
                Node(self.0 - 4, self.1, hist),
                self.heat_loss_to(self.0 - 4, self.1, map),
            ));
        }
        // North
        let mut hist = hist_o.clone();
        if *self.2.last().unwrap() == (0, -1) && sy > -7 && self.1 > 0 {
            hist.push((0, -1));
            res.push((
                Node(self.0, self.1 - 1, hist),
                map[self.1 as usize - 1][self.0 as usize],
            ));
        } else if sy > -7 && self.1 - 4 >= 0 && *self.2.last().unwrap() != (0, 1) {
            hist.push((0, -1));
            res.push((
                Node(self.0, self.1 - 4, hist),
                self.heat_loss_to(self.0, self.1 - 4, map),
            ));
        }
        // South
        let mut hist = hist_o.clone();
        if *self.2.last().unwrap() == (0, 1) && sy < 7 && self.1 + 1 < map.len() as i32 {
            hist.push((0, 1));
            res.push((
                Node(self.0, self.1 + 1, hist),
                map[self.1 as usize + 1][self.0 as usize],
            ));
        } else if sy < 7 && self.1 + 4 < map.len() as i32 && *self.2.last().unwrap() != (0, -1) {
            hist.push((0, 1));
            res.push((
                Node(self.0, self.1 + 4, hist),
                self.heat_loss_to(self.0, self.1 + 4, map),
            ));
        }
        // println!("{},{} => {res:?}", self.0, self.1);

        res
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<_>())
        .collect::<_>()
}

#[aoc(day17, part1)]
fn part1(input: &ParsedInput) -> u32 {
    let path = astar(
        &Node(0, 0, smallvec![(0, 0); 3]),
        |n| n.successors(input, input.len()),
        |c| c.0.abs_diff(input.len() as i32) + c.1.abs_diff(input.len() as i32),
        |p| p.0 == input.len() as i32 - 1 && p.1 == input.len() as i32 - 1,
    )
    .unwrap();

    path.1
}

#[aoc(day17, part2)]
fn part2(input: &ParsedInput) -> u32 {
    let path = astar(
        &Node(0, 0, smallvec![(0, 0); 7]),
        |n| n.successors_p2(input),
        |c| c.0.abs_diff(input.len() as i32) + c.1.abs_diff(input.len() as i32),
        |p| p.0 == input[0].len() as i32 - 1 && p.1 == input.len() as i32 - 1,
    )
    .unwrap();
    let mut v = HashSet::new();
    v.extend(path.0.iter().map(|n| (n.0, n.1)));
    // for (y, l) in input.iter().enumerate() {
    //     for (x, n) in l.iter().enumerate() {
    //         if v.contains(&(x as i32, y as i32)) {
    //             print!("X");
    //         } else {
    //             print!("{n}");
    //         }
    //     }
    //     println!();
    // }
    // println!("{:?}", path.0);
    path.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 94);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(&parse(
                r"111111111111
999999999991
999999999991
999999999991
999999999991"
            )),
            71
        );
    }
}
