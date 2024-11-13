use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 25)
}

fn part1(input: &str) -> i64 {
    let c_re = regex!(r#"cpy (\d+) c"#);
    let b_re = regex!(r#"cpy (\d+) b"#);

    let c: i64 = c_re.captures(input).unwrap()[1].parse().unwrap();
    let b: i64 = b_re.captures(input).unwrap()[1].parse().unwrap();

    let min = c * b;
    let mut val = 0;

    while val < min {
        val = val * 2 + 1;
        val *= 2;
    }

    val - min

    // c = ??
    // b = ??
    // loop {
    //     let x = a + c * b
    //     while x != 0 {
    //         out x % 2
    //         x = x / 2
    //     }
    // }
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 192);
}
