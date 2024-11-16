mod crypto;
mod grid;
mod iterators;
mod range;
mod strings;
mod vectors;

pub use crypto::*;
pub use grid::*;
pub use iterators::*;
pub use range::*;
pub use strings::*;
pub use vectors::*;

pub use itertools::{chain, iproduct, Itertools as _};
pub use memoize::memoize;
pub use rustc_hash::FxHashMap as HashMap;
pub use rustc_hash::FxHashSet as HashSet;

#[macro_export]
macro_rules! include_input {
    ($extra:literal / $year:literal / $day:literal) => {{
        concat!(
            $extra,
            include_str!(concat!(
                "../../input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ))
        )
    }};
    ($year:literal / $day:literal) => {{
        include_str!(concat!(
            "../../input/",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ))
    }};
}
