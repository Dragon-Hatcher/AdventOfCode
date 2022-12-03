use std::str::{Lines, Split};

pub trait AocParsed<'a>: Into<&'a str> {
    fn nums(self) -> NumIter<'a> {
        NumIter(self.into().lines())
    }

    fn non_empty(self) -> NonEmptyIter<'a> {
        NonEmptyIter(self.into().lines())
    }

    fn blank_separated(self) -> Split<'a, &'static str> {
        self.into().split("\n\n")
    }
}

impl<'a, I: Into<&'a str>> AocParsed<'a> for I {}

pub struct NumIter<'a>(Lines<'a>);

impl<'a> Iterator for NumIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.0.next()?;
            if let Ok(num) = line.trim().parse::<i64>() {
                return Some(num);
            }
        }
    }
}

pub struct NonEmptyIter<'a>(Lines<'a>);

impl<'a> Iterator for NonEmptyIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.0.next()?.trim();
            if !line.is_empty() {
                return Some(line);
            }
        }
    }
}
