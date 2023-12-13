use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<Vec<Vec<char>>>;

#[aoc_generator(day13)]
fn parse(input: &str) -> ParsedInput {
    input
        .split("\n\n")
        .map(|pat| pat.lines().map(|l| l.chars().collect::<_>()).collect::<_>())
        .collect::<_>()
}

fn find_reflection(pat: &[Vec<char>]) -> usize {
    for mid in 1..(pat.first().unwrap().len()) {
        if pat.iter().all(|l| {
            let (a, b) = l.split_at(mid);
            a.iter().rev().zip(b.iter()).all(|(a, b)| a == b)
        }) {
            return mid;
        }
    }
    0
}

#[aoc(day13, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut res = 0;

    for pat in input {
        res += match find_reflection(pat) as i64 {
            0 => {
                let mut rotated = vec![];
                for i in 0..pat.first().unwrap().len() {
                    rotated.push(pat.iter().map(|l| l[i]).collect::<Vec<_>>());
                }

                find_reflection(&rotated) as i64 * 100
            }
            n => n,
        }
    }

    res
}

#[aoc(day13, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let mut res = 0;

    for pat in input {
        let mut smudged = vec![];

        for y in 0..pat.len() {
            for x in 0..pat[y].len() {
                let mut pat_c = pat.clone();
                pat_c[y][x] = match pat_c[y][x] {
                    '.' => '#',
                    '#' => '.',
                    _ => unreachable!(),
                };
                smudged.push(pat_c);
            }
        }

        let orig = part1(&vec![pat.clone()]);
        for pat_sm in smudged {
            for mid in 1..(pat_sm.first().unwrap().len()) {
                if pat_sm.iter().all(|l| {
                    let (a, b) = l.split_at(mid);
                    a.iter().rev().zip(b.iter()).all(|(a, b)| a == b)
                }) && mid as i64 != orig
                {
                    res += mid as i64;
                    break;
                }
            }

            let mut pat_sm_rot = vec![];
            for i in 0..pat_sm.first().unwrap().len() {
                pat_sm_rot.push(pat_sm.iter().map(|l| l[i]).collect::<Vec<_>>());
            }
            for mid in 1..(pat_sm_rot.first().unwrap().len()) {
                if pat_sm_rot.iter().all(|l| {
                    let (a, b) = l.split_at(mid);
                    a.iter().rev().zip(b.iter()).all(|(a, b)| a == b)
                }) && mid as i64 * 100 != orig
                {
                    res += mid as i64 * 100;
                    break;
                }
            }
        }
    }

    res / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 400);
    }
}
