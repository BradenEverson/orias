pub trait Polygon{
    fn sides(&self) -> u8;
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}
