use std::{fs::read_to_string, io::stdin, process::exit};
mod standard_parsers;

// SOLUTION MODULES
mod day1;
mod day2;
mod day3;

static DAY_FNS: &[(DayFunc, DayFunc, Answers)] = &[
    // SOLUTION FUNCTIONS
    (day1::part1, day1::part2, day1::ANSWERS),
    (day2::part1, day2::part2, day2::ANSWERS),
    (day3::part1, day3::part2, day3::ANSWERS),
];

type DayFunc = fn(&str) -> i64;
type Answers = (i64, i64, i64, i64);

fn main() {
    let mut all_correct = true;

    for (day_num, (part1, part2, (part1_ex_answer, part1_answer, part2_ex_answer, part2_answer))) in
        DAY_FNS.iter().enumerate().rev()
    {
        let day_num = day_num + 1;

        let input_file_name = format!("./inputs/day{day_num}.txt");
        let test_input_file_name = format!("./inputs/day{day_num}-test.txt");

        let input = read_to_string(input_file_name).unwrap();
        let test_input = read_to_string(test_input_file_name).unwrap();

        let part1_guess = part1(&input);
        let part1_ex_guess = part1(&test_input);
        let part2_guess = part2(&input);
        let part2_ex_guess = part2(&test_input);

        all_correct = all_correct
            && part1_guess == *part1_answer
            && part1_ex_guess == *part1_ex_answer
            && part2_guess == *part2_answer
            && part2_ex_guess == *part2_ex_answer;

        const RED: &str = "\x1b[31m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";

        fn c(a: i64, b: i64) -> &'static str {
            if a == b {
                GREEN
            } else {
                RED
            }
        }

        println!("Day {day_num}:");
        println!(
            "    Part 1 Ex: {}{part1_guess:>6}{RESET} - {part1_answer:<6}",
            c(part1_guess, *part1_answer)
        );
        println!(
            "    Part 1:    {}{part1_ex_guess:>6}{RESET} - {part1_ex_answer:<6}",
            c(part1_ex_guess, *part1_ex_answer)
        );
        println!(
            "    Part 2 Ex: {}{part2_guess:>6}{RESET} - {part2_answer:<6}",
            c(part2_guess, *part2_answer)
        );
        println!(
            "    Part 2:    {}{part2_ex_guess:>6}{RESET} - {part2_ex_answer:<6}",
            c(part2_ex_guess, *part2_ex_answer)
        );

        if day_num == DAY_FNS.len() {
            _ = stdin().read_line(&mut String::new());
        } else {
            println!();
        }
    }

    exit(!all_correct as i32);
}
