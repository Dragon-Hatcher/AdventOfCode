use crate::{IterExtension, Range, Vector2};
use itertools::iproduct;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: i64,
}

impl<T> Grid<T> {
    pub fn new_by_char<F>(str: &str, f: F) -> Grid<T>
    where
        F: Fn(char) -> T,
    {
        let width = str.lines().nu().len() as i64;
        Grid {
            elements: str.chars().filter(|&c| c != '\n').map(f).collect(),
            width,
        }
    }

    pub fn new(width: i64, height: i64, t: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            elements: (0..width * height).map(|_| t.clone()).collect(),
            width,
        }
    }

    pub fn new_with<F>(width: i64, height: i64, f: F) -> Grid<T>
    where
        F: Fn(Vector2) -> T,
    {
        Grid {
            elements: iproduct!(0..width, 0..height)
                .map(|(x, y)| f(Vector2::new(x, y)))
                .collect(),
            width,
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.elements.len() as i64 / self.width
    }

    fn in_bounds_with_dim(vec2: Vector2, width: i64, height: i64) -> bool {
        vec2.x >= 0 && vec2.x < width && vec2.y >= 0 && vec2.y < height
    }

    pub fn in_bounds(&self, vec2: Vector2) -> bool {
        Self::in_bounds_with_dim(vec2, self.width(), self.height())
    }

    pub fn range_in_bounds(&self, range: Range) -> bool {
        self.in_bounds(range.top_left()) && self.in_bounds(range.bottom_right())
    }

    pub fn neighbors_with_deltas<'a>(
        &self,
        vec2: Vector2,
        deltas: &'a [Vector2],
    ) -> impl Iterator<Item = Vector2> + 'a {
        let width = self.width();
        let height = self.height();

        deltas
            .iter()
            .map(move |delta| vec2 + delta)
            .filter(move |p| Self::in_bounds_with_dim(*p, width, height))
    }

    pub fn neighbors4(&self, vec2: Vector2) -> impl Iterator<Item = Vector2> {
        const DELTAS: &[Vector2] = &[
            Vector2::new(1, 0),
            Vector2::new(-1, 0),
            Vector2::new(0, 1),
            Vector2::new(0, -1),
        ];

        self.neighbors_with_deltas(vec2, DELTAS)
    }

    pub fn neighbors8(&self, vec2: Vector2) -> impl Iterator<Item = Vector2> + '_ {
        const DELTAS: &[Vector2] = &[
            Vector2::new(1, -1),
            Vector2::new(1, 0),
            Vector2::new(1, 1),
            Vector2::new(0, -1),
            Vector2::new(0, 1),
            Vector2::new(-1, -1),
            Vector2::new(-1, 0),
            Vector2::new(-1, 1),
        ];

        self.neighbors_with_deltas(vec2, DELTAS)
    }

    pub fn range(&self) -> Range {
        Range::new_bl(Vector2::ZERO, Vector2::new(self.width(), self.height()))
    }

    pub fn points(&self) -> impl Iterator<Item = Vector2> {
        self.range().points()
    }

    pub fn row(&self, y: i64) -> Range {
        assert!(0 <= y && y <= self.height());
        Range::new_bl(Vector2::new(0, y), Vector2::new(self.width(), 1))
    }

    pub fn col(&self, x: i64) -> Range {
        assert!(0 <= x && x <= self.width());
        Range::new_bl(Vector2::new(x, 0), Vector2::new(1, self.height()))
    }

    pub fn fill_range(&mut self, t: T, range: Range)
    where
        T: Clone,
    {
        assert!(self.range_in_bounds(range));

        for p in range.points() {
            self[p] = t.clone();
        }
    }

    pub fn fill_range_with<F>(&mut self, f: F, range: Range)
    where
        F: Fn(Vector2, &T) -> T,
    {
        assert!(self.range_in_bounds(range));

        for p in range.points() {
            self[p] = f(p, &self[p]);
        }
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            elements: self.elements.iter().map(f).collect(),
            width: self.width,
        }
    }

    pub fn elements(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    pub fn rotate_range_by_delta(&mut self, range: Range, delta: Vector2)
    where
        T: Clone,
    {
        let new = Grid::new_with(range.width(), range.height(), |p| {
            let mut raw = p - delta;
            raw.x = raw.x.rem_euclid(range.width());
            raw.y = raw.y.rem_euclid(range.height());
            self[range.bottom_left() + raw].clone()
        });

        for p in new.points() {
            self[range.bottom_left() + p] = new[p].clone();
        }
    }

    pub fn rotate_right(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, Vector2::E1 * dist)
    }

    pub fn rotate_left(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, -Vector2::E1 * dist)
    }

    pub fn rotate_up(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, Vector2::E2 * dist)
    }

    pub fn rotate_down(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, -Vector2::E2 * dist)
    }

    fn calc_index(&self, vec2: Vector2) -> usize {
        (vec2.y * self.width() + vec2.x) as usize
    }
}

impl Grid<bool> {
    pub fn pretty(&self) -> String {
        let mut out = "".to_owned();

        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
                let p = Vector2::new(x, y);
                let c = if self[p] { '█' } else { ' ' };
                out.push(c);
            }
            out.push('\n');
        }

        out
    }
}

impl<T> Index<Vector2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {
        assert!(self.in_bounds(index));
        &self.elements[self.calc_index(index)]
    }
}

impl<T> IndexMut<Vector2> for Grid<T> {
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        assert!(self.in_bounds(index));
        let idx = self.calc_index(index);
        &mut self.elements[idx]
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
                let p = Vector2::new(x, y);
                write!(f, "{:?}", self[p])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
