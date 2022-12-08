use std::{fs::read_to_string, io::stdin, process::exit};

use lazy_static::lazy_static;
mod standard_parsers;

// SOLUTION MODULES
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

lazy_static! {
    static ref DAY_FNS: Vec<(DayFunc, DayFunc, Answers)> = vec![
        // SOLUTION FUNCTIONS
        (day1::part1.into(), day1::part2.into(), day1::ANSWERS),
        (day2::part1.into(), day2::part2.into(), day2::ANSWERS),
        (day3::part1.into(), day3::part2.into(), day3::ANSWERS),
        (day4::part1.into(), day4::part2.into(), day4::ANSWERS),
        (day5::part1.into(), day5::part2.into(), day5::ANSWERS),
        (day6::part1.into(), day6::part2.into(), day6::ANSWERS),
        (day7::part1.into(), day7::part2.into(), day7::ANSWERS),
        (day8::part1.into(), day8::part2.into(), day8::ANSWERS),
    ];    
}

enum DayFunc {
    Num(Box<dyn Fn(&str) -> i64 + Sync>),
    String(Box<dyn Fn(&str) -> String + Sync>),
}

trait FromFunc {
    fn from_func<F: Fn(&str) -> Self + 'static + Sync>(f: F) -> DayFunc;
}
impl FromFunc for i64 {
    fn from_func<F: Fn(&str) -> Self + 'static + Sync>(f: F) -> DayFunc {
        DayFunc::Num(Box::new(f))
    }
}
impl FromFunc for String {
    fn from_func<F: Fn(&str) -> Self + 'static + Sync>(f: F) -> DayFunc {
        DayFunc::String(Box::new(f))
    }
}

impl<T: FromFunc, F: Fn(&str) -> T + 'static + Sync> From<F> for DayFunc {
    fn from(f: F) -> Self {
        FromFunc::from_func(f)
    }
}

impl DayFunc {
    fn call(&self, input: &str) -> String {
        match self {
            DayFunc::Num(f) => f(input).to_string(),
            DayFunc::String(f) => f(input),
        }
    }
}

// type DayFunc = fn(&str) -> i64;
type Answers = (&'static str, &'static str, &'static str, &'static str);

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

        let part1_ex_guess = part1.call(&test_input);
        let part1_guess = part1.call(&input);
        let part2_ex_guess = part2.call(&test_input);
        let part2_guess = part2.call(&input);

        all_correct = all_correct
            && part1_guess == *part1_answer
            && part1_ex_guess == *part1_ex_answer
            && part2_guess == *part2_answer
            && part2_ex_guess == *part2_ex_answer;

        const RED: &str = "\x1b[31m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";

        fn check(name: &str, guess: &str, ans: &str) {
            println!(
                "    {name}: {}{guess:>9}{RESET} - {ans:<9}",
                if guess == ans { GREEN } else { RED }
            );
        }

        println!("Day {day_num}:");
        check("Part 1 Ex", &part1_ex_guess, part1_ex_answer);
        check("Part 1   ", &part1_guess, part1_answer);
        check("Part 2 Ex", &part2_ex_guess, part2_ex_answer);
        check("Part 2   ", &part2_guess, part2_answer);

        if day_num == DAY_FNS.len() {
            _ = stdin().read_line(&mut String::new());
        } else {
            println!();
        }
    }

    exit(!all_correct as i32);
}
