use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: i64,
    y: i64,
}

type ParsedInput = Vec<Galaxy>;

#[aoc_generator(day11)]
fn parse(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Galaxy {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
fn part1(input: &ParsedInput) -> i64 {
    solve(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &ParsedInput) -> i64 {
    solve(input, 1_000_000)
}

fn solve(galaxies: &ParsedInput, factor: i64) -> i64 {
    let mut galaxies: Vec<Galaxy> = galaxies.clone();
    let mut occupied_rows = vec![];
    let mut occupied_cols = vec![];

    // Insert unique x/y values into sorted arrays. Could use BinaryHeap instad but idk if it is better.
    for g in &galaxies {
        if let Err(idx) = occupied_rows.binary_search(&g.y) {
            occupied_rows.insert(idx, g.y);
        }
        if let Err(idx) = occupied_cols.binary_search(&g.x) {
            occupied_cols.insert(idx, g.x);
        }
    }

    // occupied_* vecs will always return Ok(), as they always contain the requested value
    for g in &mut galaxies {
        if let Ok(dy) = occupied_rows.binary_search(&g.y) {
            g.y += (g.y - dy as i64) * (factor - 1);
        }
        if let Ok(dx) = occupied_cols.binary_search(&g.x) {
            g.x += (g.x - dx as i64) * (factor - 1);
        }
    }

    let mut res = 0;
    for (i, g_a) in galaxies.iter().enumerate() {
        // I was using i..galaxies.len() here (and indexing), but this seems a bit cleaner
        // Not sure about performace though, I assume skipping i elements on every iteration
        // could be expensive.
        for g_b in galaxies.iter().skip(i) {
            res += g_a.x.abs_diff(g_b.x) + g_a.y.abs_diff(g_b.y);
        }
    }

    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 374);
    }
    // The website didn't give a test result, but solving p2 was easy enough to calculate it myself
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 82000210);
    }

    #[test]
    fn solve_example_10() {
        assert_eq!(solve(&parse(TESTCASE), 10), 1030);
    }

    #[test]
    fn solve_example_100() {
        assert_eq!(solve(&parse(TESTCASE), 100), 8410);
    }
}
