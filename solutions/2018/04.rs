use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 04)
}

#[derive(Debug, Clone, Default)]
struct Guard(HashMap<i64, i64>);

impl Guard {
    fn hours_asleep(&self) -> i64 {
        self.0.values().sum()
    }

    fn sleepiest_min(&self) -> (i64, i64) {
        let (hour, count) = self.0.iter().max_by_key(|(_, c)| *c).unwrap();

        (*hour, *count)
    }
}

fn find_guard_patterns(input: &str) -> HashMap<i64, Guard> {
    let mut guards: HashMap<i64, Guard> = HashMap::default();
    let mut active_guard = 0;
    let mut fell_asleep = None;

    for action in input.lines().sorted() {
        if action.contains("wakes up") {
            let (_, _, _, _, min) = action.nums().tup();
            if let Some(fell_asleep) = fell_asleep {
                let guard = guards.entry(active_guard).or_default();
                for min in fell_asleep..min {
                    *guard.0.entry(min).or_default() += 1;
                }
            }
        } else if action.contains("falls asleep") {
            let (_, _, _, _, min) = action.nums().tup();
            fell_asleep = Some(min);
        } else {
            let (_, _, _, _, _, id) = action.nums().tup();
            active_guard = id;
            fell_asleep = None;
        }
    }

    guards
}

fn part1(input: &str) -> i64 {
    let (id, guard) = find_guard_patterns(input)
        .into_iter()
        .max_by_key(|(_, g)| g.hours_asleep())
        .unwrap();

    id * guard.sleepiest_min().0
}

fn part2(input: &str) -> i64 {
    let (id, (min, _count)) = find_guard_patterns(input)
        .into_iter()
        .map(|(id, guard)| (id, guard.sleepiest_min()))
        .max_by_key(|(_, (_min, count))| *count)
        .unwrap();

    id * min
}

fn main() {
    advent::new(2018, 4, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
    assert_eq!(part1(input), 240);
    assert_eq!(part2(input), 4455);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 103720);
    assert_eq!(part2(input), 110913);
}
