use itertools::iproduct;

use crate::{v2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    top_left: Vec2,
    width: i64,
    height: i64,
}

impl Range {
    pub fn new_tl(top_left: Vec2, width: i64, height: i64) -> Self {
        Range {
            top_left,
            width,
            height,
        }
    }

    /// Both `top_left` and `bottom_right` are *inclusive*.
    pub fn new_tl_br(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self::new_tl(
            top_left,
            bottom_right.x - top_left.x + 1,
            bottom_right.y - top_left.y + 1,
        )
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }

    pub fn left(&self) -> i64 {
        self.top_left.x
    }

    pub fn top(&self) -> i64 {
        self.top_left.y
    }

    /// Right x coordinate. *Exclusive*
    pub fn right(&self) -> i64 {
        self.top_left.x + self.width
    }

    /// bottom y coordinate. *Exclusive*
    pub fn bottom(&self) -> i64 {
        self.top_left.y + self.height
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.left() && p.x < self.right() && p.y >= self.top() && p.y < self.bottom()
    }

    pub fn points(&self) -> impl Iterator<Item = Vec2> {
        iproduct!(self.top()..self.bottom(), self.left()..self.right()).map(|(y, x)| v2(x, y))
    }

    pub fn xs(&self) -> impl Iterator<Item = i64> {
        self.left()..self.right()
    }

    pub fn ys(&self) -> impl Iterator<Item = i64> {
        self.top()..self.bottom()
    }
}
