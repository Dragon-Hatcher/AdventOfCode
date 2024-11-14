use std::str::{Chars, Split};

pub trait StrExtensions<'a>: Into<&'a str> {
    fn nums(self) -> NumberIterator<'a> {
        NumberIterator {
            chars: self.into().chars(),
            positive: false,
        }
    }

    fn nums_pos(self) -> NumberIterator<'a> {
        NumberIterator {
            chars: self.into().chars(),
            positive: true,
        }
    }

    fn sections(self) -> Split<'a, &'static str> {
        self.into().split("\n\n")
    }
}

impl<'a, I: Into<&'a str>> StrExtensions<'a> for I {}

pub struct NumberIterator<'a> {
    chars: Chars<'a>,
    positive: bool,
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut started = false;
        let mut sign = 1i64;
        let mut mag = 0i64;

        while let Some(c) = self.chars.next() {
            if started && !c.is_ascii_digit() {
                break;
            }

            sign = if !started && c == '-' && !self.positive {
                -1
            } else {
                1
            };

            if c.is_ascii_digit() {
                started = true;
            }

            if started {
                mag *= 10;
                mag += c.to_digit(10).unwrap() as i64;
            }
        }

        if started {
            Some(sign * mag)
        } else {
            None
        }
    }
}
