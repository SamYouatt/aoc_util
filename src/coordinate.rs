use crate::direction::Direction;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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
            y: self.y + offset.1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Coordinate3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coordinate3 {
    pub fn new(x: isize, y: isize, z: isize) -> Coordinate3 {
        Coordinate3 { x, y, z }
    }
}
