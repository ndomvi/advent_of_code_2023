use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;

// type ParsedInput = SmallVec<[SmallVec<[char; 128]>; 128]>;
type ParsedInput = SmallVec<[SmallVec<[char; 128]>; 128]>;

#[aoc_generator(day16)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.chars().collect::<_>())
        .collect::<_>()
}

fn beam(
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    visited: &mut HashSet<((i32, i32), (i32, i32))>,
    map: &ParsedInput,
) {
    if !visited.contains(&((x, y), (dx, dy))) {
        visited.insert(((x, y), (dx, dy)));
        if let Some(l) = map.get((y + dy) as usize) {
            if let Some(c) = l.get((x + dx) as usize) {
                match (c, dx, dy) {
                    ('.', _, _) => beam((x + dx, y + dy), (dx, dy), visited, map),

                    ('/', 1, 0) => beam((x + dx, y + dy), (0, -1), visited, map),
                    ('/', -1, 0) => beam((x + dx, y + dy), (0, 1), visited, map),
                    ('/', 0, 1) => beam((x + dx, y + dy), (-1, 0), visited, map),
                    ('/', 0, -1) => beam((x + dx, y + dy), (1, 0), visited, map),

                    ('\\', 1, 0) => beam((x + dx, y + dy), (0, 1), visited, map),
                    ('\\', -1, 0) => beam((x + dx, y + dy), (0, -1), visited, map),
                    ('\\', 0, 1) => beam((x + dx, y + dy), (1, 0), visited, map),
                    ('\\', 0, -1) => beam((x + dx, y + dy), (-1, 0), visited, map),

                    ('|', 1 | -1, 0) => {
                        beam((x + dx, y + dy), (0, 1), visited, map);
                        beam((x + dx, y + dy), (0, -1), visited, map)
                    }
                    ('|', 0, 1 | -1) => beam((x + dx, y + dy), (0, dy), visited, map),

                    ('-', 1 | -1, 0) => beam((x + dx, y + dy), (dx, 0), visited, map),
                    ('-', 0, 1 | -1) => {
                        beam((x + dx, y + dy), (1, 0), visited, map);
                        beam((x + dx, y + dy), (-1, 0), visited, map)
                    }
                    e => unreachable!("{e:?}"),
                }
            }
        }
    }
}

#[aoc(day16, part1)]
fn part1(input: &ParsedInput) -> usize {
    let mut visited = HashSet::new();

    beam((-1, 0), (1, 0), &mut visited, input);
    let visited = visited
        .iter()
        .map(|(coord, _)| coord)
        .collect::<HashSet<_>>();

    visited.len() - 1
}

#[aoc(day16, part2)]
fn part2(input: &ParsedInput) -> usize {
    let max_x = input[0].len() as i32;
    let max_y = input.len() as i32;

    let mut tiles = vec![];
    for x in 0..max_x {
        let mut visited = HashSet::new();
        beam((x, -1), (0, 1), &mut visited, input);
        let visited = visited
            .iter()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>();
        tiles.push(visited.len() - 1);

        let mut visited = HashSet::new();
        beam((x, max_y), (0, -1), &mut visited, input);
        let visited = visited
            .iter()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>();
        tiles.push(visited.len() - 1);
    }

    for y in 0..max_y {
        let mut visited = HashSet::new();
        beam((-1, y), (1, 0), &mut visited, input);
        let visited = visited
            .iter()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>();
        tiles.push(visited.len() - 1);

        let mut visited = HashSet::new();
        beam((max_x, y), (-1, 0), &mut visited, input);
        let visited = visited
            .iter()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>();
        tiles.push(visited.len() - 1);
    }

    *tiles.iter().reduce(|acc, i| acc.max(i)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 51);
    }
}
