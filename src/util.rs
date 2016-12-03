use std::fmt;

#[derive(Debug, Clone)]
pub struct Coord2D {
    pub x: i32,
    pub y: i32
}

impl Coord2D {
    pub fn new(x: i32, y: i32) -> Coord2D {
        Coord2D {
            x: x,
            y: y
        }
    }

    pub fn zero() -> Coord2D {
        Coord2D::new(0, 0)
    }

    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Coord2D {
    fn eq(&self, other: &Coord2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Coord2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}