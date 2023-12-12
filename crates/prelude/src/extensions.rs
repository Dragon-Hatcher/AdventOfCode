use std::str::{Split, Lines};

pub trait IterExtension: Iterator {
    fn nu(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

pub trait DEIterExtension: DoubleEndedIterator {
    fn nbu(&mut self) -> Self::Item {
        self.next_back().unwrap()
    }
}

impl<T: ?Sized> IterExtension for T where T: Iterator {}
impl<T: ?Sized> DEIterExtension for T where T: DoubleEndedIterator {}

pub trait AocParsed<'a>: Into<&'a str> {
    fn nums_pos(self) -> NumIter<'a> {
        NumIter(self.into(), false)
    }

    fn nums(self) -> NumIter<'a> {
        NumIter(self.into(), true)
    }

    fn sections(self) -> Split<'a, &'static str> {
        self.into().split("\n\n")
    }

    fn non_empty(self) -> NonEmptyIter<'a> {
        NonEmptyIter(self.into().lines())
    }
}

impl<'a, I: Into<&'a str>> AocParsed<'a> for I {}

#[derive(Debug, Clone)]
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

macro_rules! conv_to_ident {
    ($count:ident, $replace:ident) => {
        $replace
    };
}
macro_rules! conv_to_expr {
    ($count:ident, $replace:expr) => {
        $replace
    };
}
macro_rules! peel {
    ($x:ident $y:ident) => {};
    ($x:ident $($y:ident)+) => { impl_into_tup!($($y)+); };
}
macro_rules! impl_into_tup {
    ($($x:ident)+) => {
        impl<E, I: Iterator<Item = E>> IntoTup<( $(conv_to_ident!($x, E)),+ )> for I {
            fn tup(&mut self) -> ( $(conv_to_ident!($x, E)),+ ) {
                ($(conv_to_expr!($x, self.nu())),+)
            }

            fn maybe_tup(&mut self) -> Option<( $(conv_to_ident!($x, E)),+ )> {
                Some(($(conv_to_expr!($x, self.next()?)),+))
            }
        }
        peel!($($x)+);
    };
}

impl_into_tup!(X X X X X X X X X X X);
