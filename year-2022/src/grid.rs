use itertools::Itertools;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub struct Grid<T> {
    elements: Vec<T>,
    width: i64,
}

impl<T> Grid<T> {
    #[allow(unused)]
    pub fn new_from_flat(elements: Vec<T>, width: i64) -> Grid<T> {
        Grid { elements, width }
    }

    pub fn new(iter: impl Iterator<Item = impl Iterator<Item = T>>) -> Grid<T> {
        let elements = iter.map(Itertools::collect_vec).collect_vec();
        let width = elements[0].len() as i64;
        Grid {
            elements: elements.into_iter().flatten().collect_vec(),
            width,
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.elements.len() as i64 / self.width
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width() && point.y < self.height()
    }

    pub fn neighbors4(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        const DELTAS: [Point; 4] = [
            Point::new(1, 0),
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ];

        DELTAS
            .iter()
            .map(move |delta| point + *delta)
            .filter(|p| self.in_bounds(*p))
    }

    #[allow(unused)]
    pub fn neighbors8(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        const DELTAS: [Point; 8] = [
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
        ];

        DELTAS
            .iter()
            .map(move |delta| point + *delta)
            .filter(|p| self.in_bounds(*p))
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.width())
            .cartesian_product(0..self.height())
            .map(|(x, y)| Point::new(x, y))
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            elements: self.elements.iter().map(f).collect_vec(),
            width: self.width,
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        assert!(self.in_bounds(index));
        &self.elements[(index.y * self.width() + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        assert!(self.in_bounds(index));
        let w = self.width();
        &mut self.elements[(index.y * w + index.x) as usize]
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, t) in self.elements.iter().enumerate() {
            write!(f, "{:?}", t)?;
            if i as i64 % self.width == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
