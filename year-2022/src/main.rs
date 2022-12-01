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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() < 2 {
        usage()
    }

    let Ok(day_num) = args[0].parse::<usize>() else { usage() };
    let Ok(part_num) = args[1].parse::<usize>() else { usage() };
    let test = if let Some(str) = args.get(2) {
        str == "--text" || str == "-t"
    } else {
        false
    };

    if day_num > DAY_FNS.len() {
        usage()
    }

    let (part1, part2) = DAY_FNS[day_num - 1];

    let part_fn = match part_num {
        1 => part1,
        2 => part2,
        _ => usage(),
    };

    let file_name = if test {
        format!("./src/inputs/day{day_num}-test.txt")
    } else {
        format!("./src/inputs/day{day_num}.txt")
    };
    let input = read_to_string(file_name).unwrap();
    let result = part_fn(&input);

    if test {
        println!("Day {day_num} Part {part_num} (test) = {result}");
    } else {
        println!("Day {day_num} Part {part_num} = {result}");
    }

    Ok(())
}
