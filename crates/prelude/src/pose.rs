use crate::{Vector2, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pose {
    pub pos: Vector2,
    pub dir: Direction
}

impl Pose {
    pub fn new(pos: Vector2, dir: Direction) -> Self {
        Self { pos, dir }
    }
}