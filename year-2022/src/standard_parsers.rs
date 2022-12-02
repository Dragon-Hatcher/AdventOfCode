#[allow(unused)]
pub fn nums(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.lines().map(|l| l.trim()).map(|l| l.parse().unwrap())
}

pub fn nums_opt(input: &str) -> impl Iterator<Item = Option<i64>> + '_ {
    input.lines().map(|l| l.trim()).map(|l| l.parse().ok())
}
