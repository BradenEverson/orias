use crate::core::geometry::point::Point;

use super::polygon::Polygon;

pub struct Square{
    pub len: f32,
    pub pos: Point
}

impl Square {
    pub fn new(len: f32, pos: Point) -> Self {
        Self { len,  pos }
    }
}

impl Polygon for Square {
    fn sides(&self) -> u8 {
        4
    }
    fn area(&self) -> f32 {
        self.len * self.len as f32
    }
    fn perimeter(&self) -> f32 {
        self.len * 4 as f32
    }
}
