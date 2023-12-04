use crate::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn vector(self) -> Vector2 {
        match self {
            Direction::North => Vector2::E2,
            Direction::East => Vector2::E1,
            Direction::South => -Vector2::E2,
            Direction::West => -Vector2::E1,
        }
    }

    pub fn turn(self, turn: Turn) -> Direction {
        use Direction as D;
        use Turn as T;

        match (self, turn) {
            (D::North, T::Left) => D::West,
            (D::North, T::Right) => D::East,
            (D::East, T::Left) => D::North,
            (D::East, T::Right) => D::South,
            (D::South, T::Left) => D::East,
            (D::South, T::Right) => D::West,
            (D::West, T::Left) => D::South,
            (D::West, T::Right) => D::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn from_char(char: char) -> Turn {
        match char {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Invalid turn char {char}."),
        }
    }
}