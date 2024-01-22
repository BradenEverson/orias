use super::point::Point;

pub struct Line{
    point_a: Point,
    point_b: Point,

    pub len: f32,
}

impl Line {
    pub fn new(point_a: Point, point_b: Point) -> Self {
        Line{
            point_a: point_a.clone(),
            point_b: point_b.clone(),
            len: point_a.dist_to(&point_b)
        }
    }
}
