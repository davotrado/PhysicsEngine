use crate::types::vector_2d::Vector2D;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vector2D,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vector2D, radius: f32) -> Self {
        Self { center, radius }
    }
}