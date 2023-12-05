mod crypto;
mod extensions;
mod grid;
mod math;
mod range;
mod rot;
mod vector;

pub use crypto::*;
pub use extensions::*;
pub use grid::*;
pub use math::*;
pub use range::*;
pub use rot::*;
pub use vector::*;

pub use itertools::{iproduct, Itertools as _};
pub use regex_macro::regex;
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::cmp;
pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

#[macro_export]
macro_rules! include_input {
    ($extra:literal / $year:literal / $day:literal) => {{
        concat!(
            $extra,
            include_str!(concat!(
                "../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ))
        )
    }};
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
