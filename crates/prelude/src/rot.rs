use crate::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_char(char: char) -> Direction {
        match char {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("Invalid direction char {char}."),
        }
    }

    pub fn vector(self) -> Vector2 {
        match self {
            Direction::Up => -Vector2::E2,
            Direction::Right => Vector2::E1,
            Direction::Down => Vector2::E2,
            Direction::Left => -Vector2::E1,
        }
    }

    pub fn turn(self, turn: Turn) -> Direction {
        use Direction as D;
        use Turn as T;

        match (self, turn) {
            (D::Up, T::Left) => D::Left,
            (D::Up, T::Right) => D::Right,
            (D::Right, T::Left) => D::Up,
            (D::Right, T::Right) => D::Down,
            (D::Down, T::Left) => D::Right,
            (D::Down, T::Right) => D::Left,
            (D::Left, T::Left) => D::Down,
            (D::Left, T::Right) => D::Up,
        }
    }

    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn horizontal(self) -> bool {
        matches!(self, Direction::Right | Direction::Left)
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
