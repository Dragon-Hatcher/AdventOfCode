use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 05)
}

fn part1(input: &str) -> String {
    let id = input.trim();
    let mut password = "".to_owned();

    for i in 0.. {
        let test = format!("{id}{i}");
        let md5 = format!("{:032x}", md5_str(test));

        if md5.starts_with("00000") {
            password.push(md5.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
    }

    password
}

fn part2(input: &str) -> String {
    let id = input.trim();
    let mut password = [0u8; 8];

    for i in 0.. {
        let test = format!("{id}{i}");
        let md5 = format!("{:032x}", md5_str(test));

        if md5.starts_with("00000") {
            let char = md5.as_bytes()[6];
            if let Some(pos) = md5.chars().nth(5).unwrap().to_digit(10) {
                let pos = pos as usize;
                if pos < 8 && password[pos] == 0 {
                    password[pos] = char;
                }
            }

            if password.iter().all(|&c| c != 0) {
                break;
            }
        }
    }

    password.iter().map(|&c| c as char).collect()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "4543c154");
    assert_eq!(part2(input), "1050cbbd");
}
