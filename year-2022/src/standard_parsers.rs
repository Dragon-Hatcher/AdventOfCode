use std::str::{Lines, Split};

pub trait AocParsed<'a>: Into<&'a str> {
    fn nums(self) -> NumIter<'a> {
        NumIter(self.into())
    }

    fn non_empty(self) -> NonEmptyIter<'a> {
        NonEmptyIter(self.into().lines())
    }

    fn sections(self) -> Split<'a, &'static str> {
        self.into().split("\n\n")
    }
}

impl<'a, I: Into<&'a str>> AocParsed<'a> for I {}

pub struct NumIter<'a>(&'a str);

impl<'a> Iterator for NumIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut dig_length = 0;
        loop {
            let Some(next_char) = self.0.chars().nth(dig_length) else { break; };
            if next_char.is_ascii_digit() {
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
        (self.next().unwrap(), self.next().unwrap())
    }

    fn maybe_tup(&mut self) -> Option<(E, E)> {
        Some((self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E) {
        (
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E)> {
        Some((self.next()?, self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E) {
        (
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
        )
    }

    fn maybe_tup(&mut self) -> Option<(E, E, E, E)> {
        Some((self.next()?, self.next()?, self.next()?, self.next()?))
    }
}

impl<E, I: Iterator<Item = E>> IntoTup<(E, E, E, E, E)> for I {
    fn tup(&mut self) -> (E, E, E, E, E) {
        (
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
        )
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
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
