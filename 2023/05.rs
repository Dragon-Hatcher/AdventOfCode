use advent::prelude::{*, IterExtension};

fn default_input() -> &'static str {
    include_input!(2023 / 05)
}

fn part1(input: &str) -> i64 {
    let mut secs = input.sections();
    let seeds = secs.nu().nums().collect_vec();

    let map: Vec<Vec<(i64, i64, i64)>> = secs
        .map(|s| {
            let mut l = s.lines();
            l.nu();

            l.map(|ns| ns.nums().tup()).collect_vec()
        })
        .collect_vec();

    seeds
        .iter()
        .map(|seed| {
            let mut val = *seed;

            for sec in &map {
                for (dest, source, len) in sec {
                    if val >= *source && val < source + len {
                        val = dest + (val - source);
                        break;
                    }
                }
            }

            val
        })
        .min()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let mut secs = input.sections();
    let seeds = secs.nu().nums().collect_vec();

    let map: Vec<Vec<(i64, i64, i64)>> = secs
        .map(|s| {
            let mut l = s.lines();
            l.nu();

            l.map(|ns| ns.nums().tup()).collect_vec()
        })
        .collect_vec();

    seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut seed| {
            let (start, len) = seed.tup();
            let mut ranges = HashSet::new();
            ranges.insert(*start..start + len);

            for sec in &map {

                let mut left = ranges.clone();
                let mut next = HashSet::new();

                for (dest, source, len) in sec {
                    let mut new_left = HashSet::new();

                    for r in left {
                        if r.start >= *source && r.end <= source + len {
                            next.insert(*dest + (r.start - source)..dest + (r.end - source));
                        } else if r.end <= *source || r.start >= source + len {
                            new_left.insert(r);
                        } else if r.start < *source {
                            new_left.insert(r.start..*source);
                            next.insert(*dest..dest + (r.end - source));
                        } else {
                            new_left.insert((source + len)..r.end);
                            next.insert(dest + (r.start - source)..dest + len);
                        }
                    }

                    left = new_left;
                }

                next.extend(left);
                ranges = next;
            }


            let min = ranges.iter().map(|r| r.start).min().unwrap_or_default();
            min
        })
        .min()
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

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
56 93 4";
    assert_eq!(part1(input), 35);
    assert_eq!(part2(input), 46);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 322500873);
    assert_eq!(part2(input), 108956227);
}
