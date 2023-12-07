use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};

type MapStruct = (Vec<i64>, Vec<Vec<(Range<i64>, i64, i64)>>);
#[aoc_generator(day5)]
fn parse(input: &str) -> MapStruct {
    let mut maps = input.split("\n\n");
    let seeds = maps
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let maps_parsed = maps
        .map(|map| {
            let mut out = vec![];
            let map = map.split('\n').skip(1);

            for mapping in map {
                let mut nums = mapping
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap());

                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let range = nums.next().unwrap();
                out.push((src..(src + range), dst, range));
            }
            out
        })
        .collect::<Vec<_>>();
    (seeds, maps_parsed)
}

#[aoc(day5, part1)]
fn part1((seeds, maps): &MapStruct) -> Result<i64, Box<dyn std::error::Error>> {
    let mut res = i64::MAX;

    for seed in seeds {
        // Fuck me in the ass, borrow checker
        let mut seed = *seed;
        for map in maps {
            if let Some((s, d, _)) = map.iter().find(|(s, _, _)| s.contains(&seed)) {
                seed = d + (seed - s.start);
            }
        }
        res = res.min(seed);
    }

    Ok(res)
}

fn map_range(range: Range<i64>, map: &Vec<(Range<i64>, i64, i64)>) -> Vec<Range<i64>> {
    let mut out = vec![];

    for (s, dst, r) in map {
        if s.contains(&range.start) && s.contains(&range.end) {
            // Fully inside
            out.push((dst + range.start - s.start)..(dst + range.end - s.start));
        } else if s.contains(&range.start) {
            // Start inside
            out.push((dst + range.start - s.start)..(dst + r));
            out.append(&mut map_range((s.end + 1)..(range.end), map));
        } else if s.contains(&range.end) {
            //TODO: HUH? why is there a deref?
            out.push(*(dst)..(dst + range.end - s.start));
            out.append(&mut map_range((range.start)..(s.start - 1), map));
        }
    }

    if out.is_empty() {
        vec![range]
    } else {
        out
    }
}

#[aoc(day5, part2)]
fn part2((seeds, maps): &MapStruct) -> Result<i64, Box<dyn std::error::Error>> {
    let mut seed_ranges = seeds
        .chunks(2)
        .map(|seed| seed[0]..(seed[0] + seed[1]))
        .collect::<Vec<_>>();
    for map in maps {
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|range| map_range(range.clone(), map))
            .collect::<Vec<_>>();
    }

    Ok(seed_ranges
        .iter()
        .fold(i64::MAX, |acc, seed| acc.min(seed.start)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 46);
    }
}
