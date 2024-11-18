use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 04)
}

type Room<'a> = (&'a str, i64, &'a str);

fn parse(line: &str) -> Room {
    let (chars, check) = line.rsplit_once('-').unwrap();
    let (sec_id, check) = check.split_once('[').unwrap();
    let sec_id: i64 = sec_id.parse().unwrap();
    let check = check.strip_suffix(']').unwrap();
    (chars, sec_id, check)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse)
        .filter(|room| {
            let mut counts = room.0.chars().count_occurrences();
            counts.remove(&'-');

            let expected: String = counts
                .into_iter()
                .sorted_by_key(|(char, _)| *char)
                .sorted_by_key(|(_, count)| -count)
                .map(|(c, _)| c)
                .take(5)
                .collect();

            expected == room.2
        })
        .map(|room| room.1)
        .sum()
}

fn part2(input: &str) -> i64 {
    fn rotate_char(char: char, amount: i64) -> char {
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
        .map(parse)
        .find_map(|(chars, sec_id, _)| {
            let decrypted: String = chars.chars().map(|c| rotate_char(c, sec_id)).collect();

            if decrypted == "northpole object storage" {
                Some(sec_id)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    advent::new(2016, 04, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
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
