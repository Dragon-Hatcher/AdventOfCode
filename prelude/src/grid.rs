use itertools::iproduct;

use crate::{hashmap, v2, IterExtensions, Range, Vec2};
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

    pub fn row_range(&self, y: i64) -> Range {
        assert!(0 <= y && y < self.height());
        Range::new_tl(Vec2::new(0, y), self.width(), 1)
    }

    pub fn col_range(&self, x: i64) -> Range {
        assert!(0 <= x && x < self.width());
        Range::new_tl(Vec2::new(x, 0), 1, self.height())
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

    pub fn neighbors_with_deltas<'a>(
        &self,
        p: Vec2,
        deltas: &'a [Vec2],
    ) -> impl Iterator<Item = Vec2> + 'a {
        let range = self.range();

        deltas
            .iter()
            .map(move |d| p + d)
            .filter(move |&p| range.contains(p))
    }

    pub fn neighbors4(&self, p: Vec2) -> impl Iterator<Item = Vec2> {
        const DELTAS: &[Vec2] = &[
            Vec2::new(1, 0),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(0, -1),
        ];

        self.neighbors_with_deltas(p, DELTAS)
    }

    pub fn neighbors8(&self, p: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        const DELTAS: &[Vec2] = &[
            Vec2::new(1, -1),
            Vec2::new(1, 0),
            Vec2::new(1, 1),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(-1, -1),
            Vec2::new(-1, 0),
            Vec2::new(-1, 1),
        ];

        self.neighbors_with_deltas(p, DELTAS)
    }

    pub fn mirror_vertical(&self) -> Grid<T>
    where
        T: Clone,
    {
        let height = self.height();
        Grid::new_with(self.width(), self.height(), |p| {
            self[v2(p.x, height - p.y - 1)].clone()
        })
    }

    pub fn mirror_horizontal(&self) -> Grid<T>
    where
        T: Clone,
    {
        let width = self.width();
        Grid::new_with(self.width(), self.height(), |p| {
            self[v2(width - p.x - 1, p.y)].clone()
        })
    }

    pub fn rotate_90(&self) -> Grid<T>
    where
        T: Clone,
    {
        let height = self.height();
        Grid::new_with(self.height(), self.width(), |p| {
            self[v2(height - p.y - 1, p.x)].clone()
        })
    }

    pub fn sub_grid(&self, range: Range) -> Grid<T>
    where
        T: Clone,
    {
        Grid::new_with(range.width(), range.height(), |p| {
            self[range.top_left() + p].clone()
        })
    }

    pub fn rotate_range_by_delta(&mut self, range: Range, delta: Vec2)
    where
        T: Clone,
    {
        let new = Grid::new_with(range.width(), range.height(), |p| {
            let mut raw = p - delta;
            raw.x = raw.x.rem_euclid(range.width());
            raw.y = raw.y.rem_euclid(range.height());
            self[range.top_left() + raw].clone()
        });

        for p in new.points() {
            self[range.top_left() + p] = new[p].clone();
        }
    }

    pub fn rotate_right(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, Vec2::E1 * dist)
    }

    pub fn rotate_left(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, -Vec2::E1 * dist)
    }

    pub fn rotate_up(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, -Vec2::E2 * dist)
    }

    pub fn rotate_down(&mut self, r: Range, dist: i64)
    where
        T: Clone,
    {
        self.rotate_range_by_delta(r, Vec2::E2 * dist)
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(&T) -> U,
    {
        Grid::new_with(self.width(), self.height(), |p| f(&self[p]))
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
    pub fn count_true(&self) -> i64 {
        self.elements().filter(|e| **e).count() as i64
    }

    pub fn pretty(&self) -> String {
        let mut out = "".to_owned();

        for y in self.range().ys() {
            for x in self.range().xs() {
                let p = Vec2 { x, y };
                let c = if self[p] { "⬤ " } else { "＊" };
                out.push_str(c);
            }
            out.push('\n');
        }

        out
    }

    pub fn parse_char(&self, col: i64) -> Option<char> {
        let font = hashmap!(
            0b01100_10010_10010_11110_10010_10010 => 'A',
            0b11100_10010_11100_10010_10010_11100 => 'B',
            0b11110_10000_11100_10000_10000_10000 => 'F',
            0b00110_00010_00010_00010_10010_01100 => 'J',
            0b11100_10010_10010_11100_10000_10000 => 'P',
            0b01110_10000_10000_01100_00010_11100 => 'S',
            0b10010_10010_10010_10010_10010_01100 => 'U',
            0b11110_00010_00100_01000_10000_11110 => 'Z',
        );

        let mut key = 0;
        for y in 0..6 {
            for x in col..col + 5 {
                key <<= 1;
                key |= self[v2(x, y)] as i32;
            }
        }

        font.get(&key).copied()
    }

    pub fn parse_str(&self) -> String {
        let mut parsed = String::new();
        let mut col = 0;
        while col + 4 < self.width() {
            parsed.push(self.parse_char(col).unwrap_or('?'));
            col += 5;
        }
        parsed
    }
}
