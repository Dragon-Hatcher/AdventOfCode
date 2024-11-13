use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 04)
}

fn parse(line: &str) -> (&str, i64, &str) {
    let (chars, check) = line.rsplit_once('-').unwrap();
    let (sec_id, check) = check.split_once('[').unwrap();
    let sec_id: i64 = sec_id.parse().unwrap();
    let check = check.strip_suffix(']').unwrap();
    (chars, sec_id, check)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (chars, sec_id, check) = parse(l);

            let mut occurrence_count: FxHashMap<char, i64> = FxHashMap::default();
            for c in chars.chars().filter(|&c| c != '-') {
                *occurrence_count.entry(c).or_default() += 1;
            }

            let passes = occurrence_count
                .into_iter()
                .sorted_by_key(|(char, _)| *char)
                .sorted_by_key(|(_, count)| -count)
                .take(5)
                .map(|(c, _)| c)
                .all(|c| check.contains(c));

            if passes {
                sec_id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    fn rotate_letter(char: char, amount: i64) -> char {
        if char == '-' {
            ' '
        } else {
            let idx = char as i64 - 'a' as i64;
            let idx = (idx + amount) % 26;
            ('a' as i64 + idx) as u8 as char
        }
    }

    input
        .lines()
        .find_map(|l| {
            let (chars, sec_id, _) = parse(l);
            let decrypted: String = chars.chars().map(|c| rotate_letter(c, sec_id)).collect();

            if decrypted == "northpole object storage" {
                Some(sec_id)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
    assert_eq!(part1(input), 1514);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 158835);
    assert_eq!(part2(input), 993);
}
