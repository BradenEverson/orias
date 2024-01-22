use std::ops;

#[derive(Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Point {
            x,
            y
        }
    }
    pub fn dist_to(&self, other: &Point) -> f32 {
        ((self.x + other.x).powi(2) + (self.y + other.y).powi(2)).sqrt()
    }
}
