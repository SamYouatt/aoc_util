pub enum Direction {
    North,
    South,
    East,
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
}
