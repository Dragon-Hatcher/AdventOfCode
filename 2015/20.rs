use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 20)
}

fn part1(input: &str) -> i64 {
    let goal = input.nums().nu();

    fn presents(house: i64) -> i64 {
        let mut sum = house + 1;
        for divisor in (2..).take_while(|x| x * x <= house) {
            if house % divisor == 0 {
                sum += divisor;
                if divisor * divisor != house {
                    sum += house / divisor;
                }
            }
        }
        sum * 10
    }

    for house in 2.. {
        if presents(house) >= goal {
            return house;
        }
    }

    panic!();
}

fn part2(input: &str) -> i64 {
    let goal = input.nums().nu();

    fn presents(house: i64) -> i64 {
        let mut sum = house + 1;
        for divisor in (2..).take_while(|x| x * x <= house) {
            if house % divisor == 0 {
                if house <= divisor * 50 {
                    sum += divisor;
                }
                if divisor * divisor != house && house <= (house / divisor) * 50 {
                    sum += house / divisor;
                }
            }
        }
        sum * 11
    }

    for house in 2.. {
        if presents(house) >= goal {
            return house;
        }
    }

    panic!();
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 665280);
    assert_eq!(part2(input), 705600);
}
