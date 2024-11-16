use itertools::iproduct;

use crate::{IterExtensions, Range, Vec2};
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    width: usize,
    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new_with<F>(width: i64, height: i64, f: F) -> Grid<T>
    where
        F: Fn(Vec2) -> T,
    {
        Grid {
            width: width as usize,
            elements: iproduct!(0..height, 0..width)
                .map(|(y, x)| f(Vec2 { x, y }))
                .collect(),
        }
    }

    pub fn new_homogenous(width: i64, height: i64, t: T) -> Grid<T>
    where
        T: Clone,
    {
        Self::new_with(width, height, |_| t.clone())
    }

    pub fn new_by_char<F>(str: &str, f: F) -> Grid<T>
    where
        F: Fn(char) -> T,
    {
        let width = str.lines().nu().len();
        Grid {
            width,
            elements: str.chars().filter(|&c| c != '\n').map(f).collect(),
        }
    }

    pub fn width(&self) -> i64 {
        self.width as i64
    }

    pub fn height(&self) -> i64 {
        (self.elements.len() / self.width) as i64
    }

    pub fn range(&self) -> Range {
        Range::new_tl(Vec2::ZERO, self.width(), self.height())
    }

    pub fn in_bounds(&self, p: Vec2) -> bool {
        self.range().contains(p)
    }

    pub fn points(&self) -> impl Iterator<Item = Vec2> {
        self.range().points()
    }

    pub fn elements(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    pub fn fill_range_with<F>(&mut self, r: Range, f: F)
    where
        F: Fn(Vec2, &T) -> T,
    {
        for p in r.points() {
            self[p] = f(p, &self[p]);
        }
    }

    pub fn fill_range(&mut self, r: Range, t: T)
    where
        T: Clone,
    {
        self.fill_range_with(r, |_, _| t.clone());
    }

    fn get_index(&self, cell: Vec2) -> usize {
        cell.y as usize * self.width + cell.x as usize
    }
}

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, cell: Vec2) -> &Self::Output {
        &self.elements[self.get_index(cell)]
    }
}

impl<T> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, cell: Vec2) -> &mut Self::Output {
        let index = self.get_index(cell);
        &mut self.elements[index]
    }
}

impl Grid<bool> {
    pub fn pretty(&self) -> String {
        let mut out = "".to_owned();

        for y in self.range().ys() {
            for x in self.range().xs() {
                let p = Vec2 { x, y };
                let c = if self[p] { '█' } else { '.' };
                out.push(c);
            }
            out.push('\n');
        }

        out
    }
}
