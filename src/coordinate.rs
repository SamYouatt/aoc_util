use crate::direction::Direction;

pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn move_dir(&self, dir: &Direction) -> Coordinate {
        let offset = dir.get_offset();

        Coordinate {
            x: self.x + offset.0,
            y: self.y + offset.0,
        }
    }
}
