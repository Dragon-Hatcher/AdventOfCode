use std::iter::once;
use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 06)
}

fn part1(input: &str) -> i64 {
    let lines = input.lines().collect_vec();
    let ops = lines.last().unwrap();
    let nums = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| l.nums().collect_vec())
        .collect_vec();

    let mut grand_total = 0;
    for (i, op) in ops.split_ascii_whitespace().enumerate() {
        let mut total = match op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };
        for row in &nums {
            match op {
                "+" => total += row[i],
                "*" => total *= row[i],
                _ => unreachable!(),
            }
        }
        grand_total += total;
    }

    grand_total
}

fn part2(input: &str) -> i64 {
    let lines = input.lines().collect_vec();
    let ops_line = lines.last().unwrap();

    let (indices, ops): (Vec<usize>, Vec<char>) = ops_line
        .char_indices()
        .filter(|(_, c)| !c.is_ascii_whitespace())
        .unzip();

    let mut cols = vec![Vec::new(); ops.len()];

    for (i, (a, b)) in indices.iter().copied().chain(once(ops_line.len() + 1)).tuple_windows().enumerate() {
        for row in lines.iter().take(lines.len() - 1){
            let n = &row[a..b-1];
            cols[i].push(n);
        }
    }

    let mut grand_total = 0;
    for (op, col) in ops.iter().zip(cols) {
        let mut total = match *op {
            '+' => 0,
            '*' => 1,
            _ => unreachable!(),
        };

        for i in 0..col[0].len() {
            let mut n = 0;
            for d in &col {
                let d = d.as_bytes()[i];
                if n != 0 && d == b' ' {
                    break;
                }

                n *= 10;
                n += (if d == b' ' { 0 } else {d - b'0'}) as i64;
            }
            match *op {
                '+' => total += n,
                '*' => total *= n,
                _ => unreachable!(),
            }
        }

        grand_total += total;
    }

    grand_total
}

fn main() {
    advent::new(2025, 6, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part1(input), 4277556);
    assert_eq!(part2(input), 3263827);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
