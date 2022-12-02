use std::{fs::read_to_string, process::exit};
mod get_data;
mod standard_parsers;

mod day1;

static DAY_FNS: &[(DayFunc, DayFunc)] = &[(day1::part1, day1::part2)];

type DayFunc = fn(&str) -> i64;

fn usage() -> ! {
    println!("USAGE: year-2022 [DAY_NUM] [PART_NUM]");

    exit(1)
}

fn is_test(args: &[String]) -> bool {
    args.iter().any(|a| *a == "--text" || *a == "-t")
}

fn get_get_data(args: &[String]) -> Option<usize> {
    if args.len() == 2 && (args[0] == "--get-date" || args[0] == "-d") {
        args[1].parse().ok()
    } else {
        None
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Some(day) = get_get_data(&args) {
        get_data::get_data(day)?;
        println!("Successfully added new day.");
        return Ok(());
    }

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
    } else if test {
        let result1 = part1(&input);
        println!("Day {day_num} Part 1 (test) = {result1}");
        let result2 = part2(&input);
        println!("Day {day_num} Part 2 (test) = {result2}");
    } else {
        let result1 = part1(&input);
        println!("Day {day_num} Part 1 = {result1}");
        let result2 = part2(&input);
        println!("Day {day_num} Part 2 = {result2}");
    }

    Ok(())
}
