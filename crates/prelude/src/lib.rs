mod grid;
mod vector;
mod extensions;
mod math;

pub use grid::*;
pub use vector::*;
pub use extensions::*;
pub use math::*;

pub use itertools::{iproduct, Itertools as _};
pub use regex_macro::regex;
pub use std::cmp;
pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
pub use rustc_hash::{FxHashSet, FxHashMap};

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
