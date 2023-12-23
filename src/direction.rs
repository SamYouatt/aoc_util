#[derive(PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    pub fn iter() -> DirectionIter {
        DirectionIter {
            current: Direction::North,
        }
    }
}

pub struct DirectionIter {
    current: Direction,
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Direction::North => Some(Direction::East),
            Direction::East => Some(Direction::South),
            Direction::South => Some(Direction::West),
            Direction::West => None,
        }
    }
}
