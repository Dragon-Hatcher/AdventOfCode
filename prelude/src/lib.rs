mod iterators;
mod strings;

pub use iterators::*;
pub use strings::*;

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
