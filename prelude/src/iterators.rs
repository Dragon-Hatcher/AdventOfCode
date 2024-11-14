pub trait IterExtensions: Iterator {
    fn nu(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

pub trait DoubleEndedIterExtensions: DoubleEndedIterator {
    fn nbu(&mut self) -> Self::Item {
        self.next_back().unwrap()
    }
}

impl<T: ?Sized> IterExtensions for T where T: Iterator {}
impl<T: ?Sized> DoubleEndedIterExtensions for T where T: DoubleEndedIterator {}

pub trait IntoTuple<T> {
    fn tup(&mut self) -> T {
        self.maybe_tup().unwrap()
    }

    fn maybe_tup(&mut self) -> Option<T>;
}


macro_rules! convert_to_ident {
    ($count:ident, $replace:ident) => {
        $replace
    };
}
macro_rules! convert_to_expr {
    ($count:ident, $replace:expr) => {
        $replace
    };
}

macro_rules! peel {
    ($x:ident $y:ident) => {};
    ($x:ident $($y:ident)+) => { impl_into_tuple!($($y)+); };
}
macro_rules! impl_into_tuple {
    ($($x:ident)+) => {
        impl<E, I: Iterator<Item = E>> IntoTuple<( $(convert_to_ident!($x, E)),+ )> for I {
            fn maybe_tup(&mut self) -> Option<( $(convert_to_ident!($x, E)),+ )> {
                Some(($(convert_to_expr!($x, self.next()?)),+))
            }
        }
        peel!($($x)+);
    };
}

impl_into_tuple!(X X X X X X X X X X X);