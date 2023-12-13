use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use regex::bytes::Regex;
use smallvec::SmallVec;

type ParsedInput = Vec<(String, Vec<u16>)>;

#[aoc_generator(day12)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            let (springs, nums) = l.split_once(' ').unwrap();
            (
                springs.to_owned(),
                nums.split(',')
                    .map(|n| n.parse::<u16>().expect("Invalid number"))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

#[aoc(day12, part1, regex)]
fn part1(input: &ParsedInput) -> i64 {
    let mut variants = 0;
    for (line, nums) in input {
        let mut cur_variants = 0;
        // println!("{line} {nums:?}");
        let mut s = format!(".{line}.").bytes().collect::<Vec<_>>();
        let mut groups = vec![];
        for n in nums {
            groups.push(format!(r"([+#]{{{n}}})"));
        }
        let re = Regex::new(&format!("^[^+#]*{}[^+#]*$", groups.join("[^+#]+"))).unwrap();

        let mut cur = 0;
        while cur != s.len() {
            s[cur] = match s[cur] {
                b'?' => b'+',
                b'+' => b'?',
                c => c,
            };
            if s[cur] == b'+' {
                cur = 0;
                if re.is_match(&s) {
                    cur_variants += 1;
                }
            }

            cur += 1;
        }

        // I hate edge cases.
        // Not placing any springs is a single placement variant
        if cur_variants == 0 {
            cur_variants = 1;
        }
        variants += cur_variants;
        // println!("Done {line}");
    }

    variants
}

#[aoc(day12, part1)]
fn part1_recursive(input: &ParsedInput) -> i64 {
    input
        .iter()
        .map(|(springs, groups)| {
            solve(
                springs.clone().chars().collect::<SmallVec<_>>(),
                groups.clone().into(),
            )
        })
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &ParsedInput) -> i64 {
    part1_recursive(
        &input
            .iter()
            .map(|(springs, nums)| {
                let mut springs = springs.clone();
                springs.push('?');
                springs = springs.repeat(5);
                springs.pop();
                (springs, nums.repeat(5))
            })
            .collect::<Vec<_>>(),
    )
}

#[cached]
fn solve(mut springs: SmallVec<[char; 64]>, mut groups: SmallVec<[u16; 8]>) -> i64 {
    // Groups are satisfied, no more broken springs
    if groups.is_empty() {
        if springs.iter().all(|c| *c == '.' || *c == '?') {
            return 1;
        }
        return 0;
    }

    match springs.pop() {
        Some('?') => {
            let res = solve(springs.clone(), groups.clone());
            let g = groups.pop().unwrap() - 1;
            if g as usize > springs.len() {
                return 0;
            }
            let (springs, cur) = springs.split_at(springs.len() - g as usize);
            let mut springs = SmallVec::from(springs);
            if cur.iter().all(|c| *c == '#' || *c == '?') {
                if let Some('.' | '?') | None = springs.pop() {
                    return res + solve(springs, groups.clone());
                }
            }
            return res;
        }
        Some('#') => {
            let g = groups.pop().unwrap() - 1;
            if g as usize > springs.len() {
                return 0;
            }
            let (springs, cur) = springs.split_at(springs.len() - g as usize);
            let mut springs = SmallVec::from(springs);
            if cur.iter().all(|c| *c == '#' || *c == '?') {
                if let Some('.' | '?') | None = springs.pop() {
                    return solve(springs, groups.clone());
                }
            }

            return 0;
        }
        Some('.') => return solve(springs, groups),
        Some(c) => unreachable!("Unknown char: {c}"),
        None => {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TESTCASE: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 21);
    }

    #[test]
    fn part1_recursive_example() {
        assert_eq!(part1_recursive(&parse(TESTCASE)), 21);
    }

    #[test]
    fn part1_recursive_micro() {
        // I love debugging recursive functions. Best thing ever.
        assert_eq!(part1_recursive(&parse("# 1")), 1);
        assert_eq!(part1_recursive(&parse("? 1")), 1);
        assert_eq!(part1_recursive(&parse(".? 1")), 1);
        assert_eq!(part1_recursive(&parse("#? 1")), 1);
        assert_eq!(part1_recursive(&parse("?.? 1")), 2);
        assert_eq!(part1_recursive(&parse("?.? 1,1")), 1);
        assert_eq!(part1_recursive(&parse("#.? 1,1")), 1);

        assert_eq!(part1_recursive(&parse("???.### 1,1,3")), 1);
        assert_eq!(part1_recursive(&parse(".??..??...?##. 1,1,3")), 4);
        assert_eq!(part1_recursive(&parse("??.? 1,1")), 2);
        assert_eq!(part1_recursive(&parse("???? 1")), 4);
        assert_eq!(part1_recursive(&parse("??? 1,1")), 1);
        assert_eq!(part1_recursive(&parse("???? 1,1")), 3);
        assert_eq!(part1_recursive(&parse("?#?? 1,1")), 1);
        assert_eq!(part1_recursive(&parse("?###???????? 3,2,1")), 10);
        assert_eq!(part1_recursive(&parse("?#?? 1")), 1);
        assert_eq!(part1_recursive(&parse("?#???#?#?? 6,1")), 1);
        assert_eq!(part1_recursive(&parse("???##### 1,6")), 1);
        assert_eq!(part1_recursive(&parse("???#???#?#?? 1,6,1")), 1);
        assert_eq!(part1_recursive(&parse("?#????#???#?#?? 1,1,6,1")), 1);

        assert_eq!(part1_recursive(&parse("?##?.??.??? 2,1,2")), 4);
        assert_eq!(part1_recursive(&parse("?#?#????????? 2,2,4,1")), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 525152);
    }
}
