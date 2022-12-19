use lazy_static::lazy_static;
use std::{
    fs::read_to_string,
    io::stdin,
    process::exit,
    time::{Duration, Instant},
};

mod grid;
mod helpers;
mod standard_parsers;

// SOLUTION MODULES
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day16;
mod day17;

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
        (day9::part1.into(), day9::part2.into(), day9::ANSWERS),
        (day10::part1.into(), day10::part2.into(), day10::ANSWERS),
        (day11::part1.into(), day11::part2.into(), day11::ANSWERS),
        (day12::part1.into(), day12::part2.into(), day12::ANSWERS),
        (day13::part1.into(), day13::part2.into(), day13::ANSWERS),
        (day14::part1.into(), day14::part2.into(), day14::ANSWERS),
        (day15::part1.into(), day15::part2.into(), day15::ANSWERS),
        (day16::part1.into(), day16::part2.into(), day16::ANSWERS),
        (day17::part1.into(), day17::part2.into(), day17::ANSWERS),
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

const MAX_TIME: Duration = Duration::from_millis(250);

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

// type DayFunc = fn(&str) -> i64;
type Answers = (&'static str, &'static str, &'static str, &'static str);

fn get_inputs(day_num: usize) -> (String, String) {
    let input_file_name = format!("./inputs/day{day_num}.txt");
    let test_input_file_name = format!("./inputs/day{day_num}-test.txt");

    let input = read_to_string(input_file_name).unwrap();
    let test_input = read_to_string(test_input_file_name).unwrap();

    (input, test_input)
}

fn check(
    name: &str,
    ans_fn: &DayFunc,
    input: &str,
    ans: &str,
    all_correct: &mut bool,
    total_dir: &mut Duration,
) {
    let start = Instant::now();
    let guess = ans_fn.call(input);
    let elapsed = start.elapsed();
    println!(
        "    {:<10} {}{guess:>14}{RESET} - {ans:<15} {}{elapsed:.2?}{RESET}",
        format!("{name}:"),
        if guess == ans { GREEN } else { RED },
        if elapsed > MAX_TIME { RED } else { GREEN }
    );

    *all_correct = *all_correct && guess == ans;
    *total_dir += elapsed;
}

fn check_no_time(name: &str, ans_fn: &DayFunc, input: &str, ans: &str, all_correct: &mut bool) {
    let guess = ans_fn.call(input);
    println!(
        "    {:<10} {}{guess:>14}{RESET} - {ans:<15}",
        format!("{name}:"),
        if guess == ans { GREEN } else { RED },
    );

    *all_correct = *all_correct && guess == ans;
}

fn test_day(
    day_num: usize,
    (part1, part2): (&DayFunc, &DayFunc),
    (part1_ex_answer, part1_answer, part2_ex_answer, part2_answer): (&str, &str, &str, &str),
    do_examples: bool,
    pause: bool,
    all_correct_out: &mut bool,
    total_dir_out: &mut Duration,
) {
    let (input, test_input) = get_inputs(day_num);

    println!("Day {day_num}:");
    let mut all_correct = true;
    let mut total_dir = Duration::from_secs(0);
    if do_examples {
        check(
            "Part 1 Ex",
            part1,
            &test_input,
            part1_ex_answer,
            &mut all_correct,
            &mut total_dir,
        );
    }
    check(
        "Part 1",
        part1,
        &input,
        part1_answer,
        &mut all_correct,
        &mut total_dir,
    );
    if do_examples {
        check(
            "Part 2 Ex",
            part2,
            &test_input,
            part2_ex_answer,
            &mut all_correct,
            &mut total_dir,
        );
    }
    check(
        "Part 2",
        part2,
        &input,
        part2_answer,
        &mut all_correct,
        &mut total_dir,
    );

    *all_correct_out = *all_correct_out && all_correct;
    *total_dir_out += total_dir;

    if pause && day_num == DAY_FNS.len() {
        _ = stdin().read_line(&mut String::new());
    } else {
        println!();
    }
}

fn benchmark_day(day_num: usize, trials: usize) {
    let (input, _) = get_inputs(day_num);
    let (part1, part2, (_, p1_answer, _, p2_answer)) = &DAY_FNS[day_num - 1];

    let mut all_correct = true;

    println!("Day {day_num}:");
    check_no_time("Part 1", part1, &input, p1_answer, &mut all_correct);
    check_no_time("Part 2", part2, &input, p2_answer, &mut all_correct);

    let mut part1_durations: Vec<Duration> = Vec::with_capacity(trials);
    let mut part2_durations: Vec<Duration> = Vec::with_capacity(trials);

    for _ in 0..trials {
        let start = Instant::now();
        part1.call(&input);
        part1_durations.push(start.elapsed());

        let start = Instant::now();
        part2.call(&input);
        part2_durations.push(start.elapsed());
    }

    fn average(durations: &[Duration]) -> Duration {
        let total: u128 = durations.iter().map(Duration::as_nanos).sum();
        Duration::from_nanos((total / durations.len() as u128) as u64)
    }

    fn explain(part_num: usize, durations: &[Duration]) {
        let avg = average(durations);
        let min = *durations.iter().min().unwrap();
        let max = *durations.iter().max().unwrap();

        println!(
            "    Part {part_num}: {}{avg:>8.2?}{RESET} {}{min:>8.2?}{RESET} {}{max:>8.2?}{RESET}",
            if avg > MAX_TIME { RED } else { GREEN },
            if min > MAX_TIME { RED } else { GREEN },
            if max > MAX_TIME { RED } else { GREEN },
        );
    }

    println!();
    println!("Benchmarks:      Avg      Min      Max");
    explain(1, &part1_durations);
    explain(2, &part2_durations);
}

fn main() {
    let certain_day = std::env::args()
        .nth(1)
        .and_then(|a| a.parse::<usize>().ok());

    if let Some(certain_day) = certain_day {
        if certain_day > 0 && certain_day <= DAY_FNS.len() {
            let trials = std::env::args()
                .nth(2)
                .and_then(|a| a.parse::<usize>().ok())
                .unwrap_or(10000);

            benchmark_day(certain_day, trials);
            exit(0);
        } else {
            println!("That day doesn't exist.");
            exit(2);
        }
    }

    let run_examples = !std::env::args().skip(1).any(|a| {
        a.trim() == "--skip-ex" || a.trim() == "-s" || a.trim() == "-sp" || a.trim() == "-ps"
    });

    let pause = !std::env::args().skip(1).any(|a| {
        a.trim() == "--no-pause" || a.trim() == "-p" || a.trim() == "-sp" || a.trim() == "-ps"
    });

    let mut all_correct = true;
    let mut total_dir = Duration::from_secs(0);
    let start_time = Instant::now();

    for (day_num, (part1, part2, answers)) in DAY_FNS.iter().enumerate().rev() {
        test_day(
            day_num + 1,
            (part1, part2),
            *answers,
            run_examples,
            pause,
            &mut all_correct,
            &mut total_dir,
        );
    }

    let elapsed = start_time.elapsed();
    println!("Total time: {total_dir:.2?} ({elapsed:.2?})");

    exit(!all_correct as i32);
}
