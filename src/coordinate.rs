pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Coordinate {
        Coordinate { x, y }
    }
}
