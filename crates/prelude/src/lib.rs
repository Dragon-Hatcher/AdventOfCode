use std::str::{Lines, Split};

#[macro_export]
macro_rules! include_input {
    ($year:literal / $day:literal) => {{
        include_str!(concat!(
            "../input/",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ))
    }};
}

pub trait IterExtension: Iterator {
    fn nu(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<T: ?Sized> IterExtension for T where T: Iterator {}
pub trait AocParsed<'a>: Into<&'a str> {
    fn nums_pos(self) -> NumIter<'a> {
        NumIter(self.into(), false)
    }

    fn nums(self) -> NumIter<'a> {
        NumIter(self.into(), true)
    }

    fn non_empty(self) -> NonEmptyIter<'a> {
        NonEmptyIter(self.into().lines())
    }

    fn sections(self) -> Split<'a, &'static str> {
        self.into().split("\n\n")
    }
}

impl<'a, I: Into<&'a str>> AocParsed<'a> for I {}

pub struct NumIter<'a>(&'a str, bool);

impl<'a> Iterator for NumIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut dig_length = 0;
        loop {
            let Some(next_char) = self.0.chars().nth(dig_length) else {
                break;
            };
            if next_char.is_ascii_digit() || (self.1 && dig_length == 0 && next_char == '-') {
                dig_length += 1;
            } else if dig_length == 0 {
                dig_length = 0;
                self.0 = &self.0[1..];
            } else {
                break;
            }
        }
        let num = self.0[..dig_length].parse::<i64>().ok();
        self.0 = &self.0[dig_length..];
        num
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

pub trait IntoTup<T> {
    fn tup(&mut self) -> T;
    fn maybe_tup(&mut self) -> Option<T>;
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E)> for I {
    fn tup(&mut self) -> (E, E) {
        (self.nu(), self.nu())
    }

    fn maybe_tup(&mut self) -> Option<(E, E)> {
        Some((self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E) {
        (self.nu(), self.nu(), self.nu())
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E)> {
        Some((self.next()?, self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E) {
        (self.nu(), self.nu(), self.nu(), self.nu())
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E)> {
        Some((self.next()?, self.next()?, self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E) {
        (self.nu(), self.nu(), self.nu(), self.nu(), self.nu())
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E, E, E, E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E, E, E, E, E, E, E, E) {
        (
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
            self.nu(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E, E, E, E, E, E, E, E, E)> {
        Some((
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
            self.next()?,
        ))
    }
}
