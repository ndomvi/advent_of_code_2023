use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid character: {c}"),
        }
    }
}

// type ParsedInput = ((usize, usize), Vec<Vec<Pipe>>);
type ParsedInput = ((usize, usize), Vec<Vec<Pipe>>);

#[aoc_generator(day10)]
fn parse(input: &str) -> ParsedInput {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    };
                    Pipe::from(c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (start, map)
}

fn find_loop(start: &(usize, usize), map: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = vec![(start.0 as i32, start.1 as i32)];

    while let Some((x, y)) = queue.pop() {
        if y >= 0
            && (y as usize) < map.len()
            && x >= 0
            && (x as usize) < map[y as usize].len()
            && !visited.contains(&(x as usize, y as usize))
        {
            visited.insert((x as usize, y as usize));
            match map[y as usize][x as usize] {
                Pipe::NS => {
                    queue.push((x, y - 1));
                    queue.push((x, y + 1));
                }
                Pipe::EW => {
                    queue.push((x + 1, y));
                    queue.push((x - 1, y));
                }
                Pipe::NE => {
                    queue.push((x, y - 1));
                    queue.push((x + 1, y));
                }
                Pipe::NW => {
                    queue.push((x, y - 1));
                    queue.push((x - 1, y));
                }
                Pipe::SW => {
                    queue.push((x, y + 1));
                    queue.push((x - 1, y));
                }
                Pipe::SE => {
                    queue.push((x, y + 1));
                    queue.push((x + 1, y));
                }
                Pipe::Ground => (),
                Pipe::Start => {
                    if y > 0 {
                        if let Some(Pipe::NS | Pipe::SW | Pipe::SE) =
                            map.get(y as usize - 1).and_then(|row| row.get(x as usize))
                        {
                            queue.push((x, y - 1));
                        }
                    }

                    if let Some(Pipe::NE | Pipe::NS | Pipe::NW) =
                        map.get(y as usize + 1).and_then(|row| row.get(x as usize))
                    {
                        queue.push((x, y + 1));
                    }
                    if x > 0 {
                        if let Some(Pipe::SE | Pipe::NE | Pipe::EW) =
                            map.get(y as usize).and_then(|row| row.get(x as usize - 1))
                        {
                            queue.push((x - 1, y));
                        }
                    }
                    if let Some(Pipe::NW | Pipe::SW | Pipe::EW) =
                        map.get(y as usize).and_then(|row| row.get(x as usize + 1))
                    {
                        queue.push((x + 1, y));
                    }
                }
            }
        }
    }

    visited
}

#[aoc(day10, part1)]
fn part1((start, map): &ParsedInput) -> usize {
    find_loop(start, map).len() / 2
}

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[aoc(day10, part2)]
fn part2((start, map): &ParsedInput) -> usize {
    let loop_cells = find_loop(start, map);
    let mut map = map.clone();
    let mut empty = HashSet::new();
    let mut queue = vec![];

    let ymax = map.len() - 1;
    for (y, line) in map.iter_mut().enumerate() {
        let xmax = line.len() - 1;
        for (x, c) in line.iter_mut().enumerate() {
            if !loop_cells.contains(&(x, y)) {
                if x == 0 || x == xmax || y == 0 || y == ymax {
                    queue.push((x as i32 * 2, y as i32 * 2));
                }
                *c = Pipe::Ground;
                empty.insert((x as i32, y as i32));
            }
        }
    }

    let mut upscaled: Vec<Vec<Pipe>> = Vec::with_capacity(map.len() * 2);
    for line in map {
        let mut cur = Vec::with_capacity(line.len() * 2);
        let mut next = Vec::with_capacity(line.len() * 2);
        for c in line {
            match c {
                Pipe::EW => {
                    cur.extend_from_slice(&[Pipe::EW, Pipe::EW]);
                    next.extend_from_slice(&[Pipe::Ground, Pipe::Ground]);
                }
                Pipe::NS => {
                    cur.extend_from_slice(&[Pipe::NS, Pipe::Ground]);
                    next.extend_from_slice(&[Pipe::NS, Pipe::Ground]);
                }
                Pipe::NE => {
                    cur.extend_from_slice(&[Pipe::NE, Pipe::EW]);
                    next.extend_from_slice(&[Pipe::Ground, Pipe::Ground]);
                }
                Pipe::NW => {
                    cur.extend_from_slice(&[Pipe::NW, Pipe::Ground]);
                    next.extend_from_slice(&[Pipe::Ground, Pipe::Ground]);
                }
                Pipe::SW => {
                    cur.extend_from_slice(&[Pipe::SW, Pipe::Ground]);
                    next.extend_from_slice(&[Pipe::NS, Pipe::Ground]);
                }
                Pipe::SE => {
                    cur.extend_from_slice(&[Pipe::SE, Pipe::EW]);
                    next.extend_from_slice(&[Pipe::NS, Pipe::Ground]);
                }
                Pipe::Ground => {
                    cur.extend_from_slice(&[Pipe::Ground, Pipe::Ground]);
                    next.extend_from_slice(&[Pipe::Ground, Pipe::Ground]);
                }
                Pipe::Start => {
                    cur.extend_from_slice(&[Pipe::NS, Pipe::EW]);
                    next.extend_from_slice(&[Pipe::EW, Pipe::NS]);
                }
            }
        }
        upscaled.push(cur);
        upscaled.push(next);
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while let Some((x, y)) = queue.pop() {
        if visited.insert((x, y)) {
            for (dx, dy) in DIRS {
                let x = x + dx;
                let y = y + dy;

                if y >= 0
                    && (y as usize) < upscaled.len()
                    && x >= 0
                    && (x as usize) < upscaled[y as usize].len()
                    && upscaled[y as usize][x as usize] == Pipe::Ground
                {
                    queue.push((x, y));
                }
            }
        }
    }

    empty
        .difference(
            &visited
                .iter()
                .filter_map(|(x, y)| {
                    if x % 2 == 0
                        && y % 2 == 0
                        && upscaled[*y as usize][*x as usize] == Pipe::Ground
                    {
                        Some((x / 2, y / 2))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>(),
        )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 4);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            part2(&parse(
                r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
            )),
            4
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(&parse(
                r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#
            )),
            4
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            part2(&parse(
                r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
            )),
            8
        );
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            part2(&parse(
                r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            )),
            10
        );
    }
}
