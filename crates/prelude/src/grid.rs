use crate::{IterExtension, Vector2};
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

    pub fn points(&self) -> impl Iterator<Item = Vector2> {
        iproduct!(0..self.width(), 0..self.height()).map(|(x, y)| Vector2::new(x, y))
    }

    pub fn subgrid_points(
        &self,
        top_left: Vector2,
        bottom_right: Vector2,
    ) -> impl Iterator<Item = Vector2> {
        assert!(self.in_bounds(top_left));
        assert!(self.in_bounds(bottom_right));

        iproduct!(top_left.x..=bottom_right.x, top_left.y..=bottom_right.y)
            .map(|(x, y)| Vector2::new(x, y))
    }

    pub fn fill_subgrid(&mut self, t: T, top_left: Vector2, bottom_right: Vector2)
    where
        T: Clone,
    {
        for p in self.subgrid_points(top_left, bottom_right) {
            self[p] = t.clone();
        }
    }

    pub fn fill_subgrid_with<F>(&mut self, f: F, top_left: Vector2, bottom_right: Vector2)
    where
        F: Fn(Vector2, &T) -> T,
    {
        for p in self.subgrid_points(top_left, bottom_right) {
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

    fn calc_index(&self, vec2: Vector2) -> usize {
        (vec2.y * self.width() + vec2.x) as usize
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
        for (i, t) in self.elements.iter().enumerate() {
            write!(f, "{:?}", t)?;
            if i as i64 % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
