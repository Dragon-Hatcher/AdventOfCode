mod crypto;
mod grid;
mod id_gen;
mod iterators;
mod math;
mod range;
mod rotations;
mod search;
mod strings;
mod vectors;

pub use crypto::*;
pub use grid::*;
pub use id_gen::*;
pub use iterators::*;
pub use math::*;
pub use range::*;
pub use rotations::*;
pub use search::*;
pub use strings::*;
pub use vectors::*;

pub use itertools::{chain, iproduct, Itertools as _};
pub use memoize::memoize;
pub use rustc_hash;
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

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = rustc_hash::FxHashMap::default();
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
