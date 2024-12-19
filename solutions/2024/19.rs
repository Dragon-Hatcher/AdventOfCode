use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 19)
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.sections().tup();
    let towels = towels.split(", ").map(|t| t.trim()).collect();
    let designs = designs.lines().map(|l| l.trim()).collect();
    (towels, designs)
}

fn part1(input: &str) -> i64 {
    let (towels, designs) = parse(input);

    fn can_do(design: &str, with: &[&str]) -> bool {
        if design.is_empty() {
            return true;
        }
    
        for towel in with {
            let Some(rest) = design.strip_prefix(towel) else { continue; };

            if can_do(rest, with) {
                return true;
            }
        }
    
        false
    }

    designs
        .into_iter()
        .filter(|d| can_do(d, &towels))
        .count()
        as i64
}

fn part2(input: &str) -> i64 {
    let (towels, designs) = parse(input);

    fn count_ways<'a>(design: &'a str, with: &[&str], memo: &mut HashMap<&'a str, i64>) -> i64 {        
        if design.is_empty() {
            return 1;
        }

        if let Some(cnt) = memo.get(design) {
            return *cnt;
        }
    
        let mut sum = 0;
        for towel in with {
            let Some(rest) = design.strip_prefix(towel) else { continue; };
            sum += count_ways(rest, with, memo);
        }
    
        memo.insert(design, sum);
        sum
    }

    let mut memo = HashMap::default();
    designs
        .into_iter()
        .map(|d| count_ways(d, &towels, &mut memo))
        .sum()
}

fn main() {
    advent::new(2024, 19, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part1(input), 6);
    assert_eq!(part2(input), 16);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 263);
    assert_eq!(part2(input), 723524534506343);
}
