use crate::primitives::{Direction, Point, Time};

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: Time) -> Point {
        self.origin + t * self.direction
    }
}

