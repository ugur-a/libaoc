#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction4 {
    Up,
    Right,
    Down,
    Left,
}

impl Direction4 {
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}
