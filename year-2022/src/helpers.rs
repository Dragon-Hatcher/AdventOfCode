pub trait I64Extension {
    fn dist(self, other: Self) -> Self;
}

impl I64Extension for i64 {
    fn dist(self, other: Self) -> Self {
        (self - other).abs()
    }
}

pub trait BoolExtension {
    fn choice<T>(self, t: T, f: T) -> T;
}

impl BoolExtension for bool {
    fn choice<T>(self, t: T, f: T) -> T {
        if self {
            t
        } else {
            f
        }
    }
}
