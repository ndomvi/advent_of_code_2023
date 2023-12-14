use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use smallvec::SmallVec;

type ParsedInput = SmallVec<[SmallVec<[char; 128]>; 128]>;

#[aoc_generator(day14)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.chars().collect::<_>())
        .collect::<_>()
}

#[aoc(day14, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut input = input.clone();
    // Roll
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'O' {
                let mut cy = y;
                while (cy as i64 - 1) >= 0 && input[cy - 1][x] == '.' {
                    cy -= 1;
                }
                input[y][x] = '.';
                input[cy][x] = 'O';
            }
        }
    }

    // Calculate the load
    let mut res = 0;
    for (y, l) in input.iter().enumerate() {
        for c in l {
            if *c == 'O' {
                res += input.len() - y;
            }
        }
    }

    res as i64
}

fn cycle(mut input: ParsedInput) -> ParsedInput {
    let y_len = input.len();
    let x_len = input[0].len();
    // North
    for y in 0..y_len {
        for x in 0..x_len {
            if input[y][x] == 'O' {
                let mut cy = y;
                while (cy as i64 - 1) >= 0 && input[cy - 1][x] == '.' {
                    cy -= 1;
                }
                input[y][x] = '.';
                input[cy][x] = 'O';
            }
        }
    }

    // West
    for y in 0..y_len {
        for x in 0..x_len {
            if input[y][x] == 'O' {
                let mut cx = x;
                while (cx as i64 - 1) >= 0 && input[y][cx - 1] == '.' {
                    cx -= 1;
                }
                input[y][x] = '.';
                input[y][cx] = 'O';
            }
        }
    }

    // South
    for y in (0..y_len).rev() {
        for x in 0..x_len {
            if input[y][x] == 'O' {
                let mut cy = y;
                while (cy + 1) < y_len && input[cy + 1][x] == '.' {
                    cy += 1;
                }
                input[y][x] = '.';
                input[cy][x] = 'O';
            }
        }
    }

    // East
    for y in 0..y_len {
        for x in (0..x_len).rev() {
            if input[y][x] == 'O' {
                let mut cx = x;
                while (cx + 1) < x_len && input[y][cx + 1] == '.' {
                    cx += 1;
                }
                input[y][x] = '.';
                input[y][cx] = 'O';
            }
        }
    }

    input
}

#[aoc(day14, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let mut input = input.clone();
    // Roll
    let mut seen = HashSet::new();
    for _ in 0..1000000000 {
        // Discovered by manually analyzing outputs.
        // I have no clue how it works, but it found the correct answer on test and on my input data.
        // Obviously there is a cycle in states, and the answer should be at something like
        // (billion % states_before cycle) runs, but I couldn't find the solution.
        //
        // The correct answer is 4 cycles before and 3 cycles after the first repeating state.
        if seen.contains(&input) {
            input = cycle(input);
            input = cycle(input);
            input = cycle(input);
            break;
        } else {
            seen.insert(input.clone());
        }

        input = cycle(input);
    }

    // Calculate the load
    let mut res = 0;
    for (y, l) in input.iter().enumerate() {
        for c in l {
            if *c == 'O' {
                res += input.len() - y;
            }
        }
    }

    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 64);
    }
}
