use seq_macro::seq;
use std::{error::Error, fs::read_to_string, process::exit};

mod standard_parsers;

seq!(N in 1..=1 {
    mod day~N;
});

seq!(N in 1..=1 {
    static DAY_FNS: &[(DayFunc, DayFunc)] = &[
        #(
            (day~N::part1, day~N::part2),
        )*
    ];
});

type DayFunc = fn(&str) -> i64;

fn usage() -> ! {
    println!("USAGE: year-2022 [DAY_NUM] [PART_NUM]");

    exit(1)
}

fn is_test(args: &[String]) -> bool {
    args.iter().any(|a| *a == "--text" || *a == "-t")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let day_num = args
        .get(0)
        .unwrap_or(&"".to_owned())
        .parse::<usize>()
        .unwrap_or(DAY_FNS.len());
    let part_num = args.get(1).unwrap_or(&"".to_owned()).parse::<usize>().ok();
    let test = is_test(&args);

    if day_num > DAY_FNS.len() {
        usage()
    }

    let (part1, part2) = DAY_FNS[day_num - 1];

    let file_name = if test {
        format!("./inputs/day{day_num}-test.txt")
    } else {
        format!("./inputs/day{day_num}.txt")
    };
    let input = read_to_string(file_name).unwrap();

    if let Some(part_num) = part_num {
        let part_fn = match part_num {
            1 => part1,
            2 => part2,
            _ => usage(),
        };

        let result = part_fn(&input);

        if test {
            println!("Day {day_num} Part {part_num} (test) = {result}");
        } else {
            println!("Day {day_num} Part {part_num} = {result}");
        }
    } else {
        let result1 = part1(&input);
        let result2 = part2(&input);

        if test {
            println!("Day {day_num} Part 1 (test) = {result1}");
            println!("Day {day_num} Part 2 (test) = {result2}");
        } else {
            println!("Day {day_num} Part 1 = {result1}");
            println!("Day {day_num} Part 2 = {result2}");
        }
    }

    Ok(())
}
