use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 22)
}

fn evolve(s: i64) -> i64 {
    let s = (s ^ (s * 64)) % 16777216;
    let s = (s ^ (s / 32)) % 16777216;
    let s = (s ^ (s * 2048)) % 16777216;
    s
}

fn part1(input: &str) -> i64 {
    let mut a = 0;
    for n in input.nums() {
        let mut s = n;
        for _ in 0..2000 {
            s = evolve(s);
        }
        a += s;
    }
    a
}

fn find_price_map(input: &[i64]) -> HashMap<u32, i64> {
    let mask = 0xffffffff;
    let mut total = HashMap::default();

    for &n in input {
        let mut s = n;
        let mut last = 0xeeeeeeee;

        let mut seen = HashSet::default();

        for _ in 0..2000 {
            let new = evolve(s);

            let diff = ((new % 10) - (s % 10)) as i8;
            last = ((last << 8) | (diff.to_le_bytes()[0] as u32)) & mask;

            if !seen.contains(&last) {
                *total.entry(last).or_default() += new % 10;
                seen.insert(last);
            }

            s = new;
        }
    }

    total
}

fn part2(input: &str) -> i64 {
    let input = input.nums().collect_vec();

    *find_price_map(&input).values().max().unwrap()
}

fn main() {
    advent::new(2024, 22, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("1 10 100 2024"), 37327623);
    assert_eq!(part2("1 2 3 2024"), 23);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 14119253575);
    assert_eq!(part2(input), 1600);
}
