use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 19)
}

fn solve(input: &str) -> (String, i64) {
    let grid = Grid::new_by_char(input, |c| c);

    let mut seen = String::new();
    let mut pos = grid.row_range(0).points().find(|&p| grid[p] == '|').unwrap();
    let mut direction = Direction::Down;

    let mut steps = 0;

    while grid[pos] != ' ' {
        steps += 1;

        if grid[pos].is_alphabetic() {
            seen.push(grid[pos]);
        }

        pos += direction.vector();

        if grid[pos] == '+' {
            let left = direction.turn(Turn::Left);
            let right = direction.turn(Turn::Right);

            let left_pos = pos + left.vector();
            let use_left = grid.in_bounds(left_pos) && grid[left_pos] != ' ';

            direction = if use_left { left } else { right };
        }
    }

    (seen, steps)

}

fn part1(input: &str) -> String {
    solve(input).0
}

fn part2(input: &str) -> i64 {
    solve(input).1
}

fn main() {
    advent::new(2017, 19, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
    assert_eq!(part1(input), "ABCDEF");
    assert_eq!(part2(input), 38);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "PVBSCMEQHY");
    assert_eq!(part2(input), 17736);
}
