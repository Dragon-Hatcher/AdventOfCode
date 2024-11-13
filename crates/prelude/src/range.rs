use itertools::iproduct;

use crate::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    bottom_left: Vector2,
    top_right: Vector2,
}

impl Range {
    pub fn new_bl(bottom_left: Vector2, size: Vector2) -> Range {
        Range {
            bottom_left,
            top_right: bottom_left + size - Vector2::new(1, 1),
        }
    }

    pub fn new_tl(top_left: Vector2, size: Vector2) -> Range {
        let bottom_left = top_left - size.y_comp() + Vector2::E2;
        Range::new_bl(bottom_left, size)
    }

    pub fn new_bl_tr(bottom_left: Vector2, top_right: Vector2) -> Range {
        Range {
            bottom_left,
            top_right,
        }
    }

    pub fn left(self) -> i64 {
        self.bottom_left.x
    }

    pub fn right(self) -> i64 {
        self.top_right.x
    }

    pub fn top(self) -> i64 {
        self.top_right.y
    }

    pub fn bottom(self) -> i64 {
        self.bottom_left.y
    }

    pub fn width(self) -> i64 {
        self.right() - self.left() + 1
    }

    pub fn height(self) -> i64 {
        self.top() - self.bottom() + 1
    }

    pub fn top_left(self) -> Vector2 {
        Vector2::new(self.left(), self.top())
    }

    pub fn bottom_left(self) -> Vector2 {
        self.bottom_left
    }

    pub fn top_right(self) -> Vector2 {
        self.top_right
    }

    pub fn bottom_right(self) -> Vector2 {
        Vector2::new(self.right(), self.bottom())
    }

    pub fn points(self) -> impl Iterator<Item = Vector2> {
        iproduct!(self.left()..=self.right(), self.bottom()..=self.top())
            .map(|(x, y)| Vector2 { x, y })
    }
}
