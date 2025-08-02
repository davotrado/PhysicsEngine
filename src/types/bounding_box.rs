use crate::types::vector_2d::Vector2D;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub center: Vector2D,
    pub half_width: f32,
    pub half_height: f32,
}

impl BoundingBox {
    pub fn new(center: Vector2D, half_width: f32, half_height: f32) -> Self {
        BoundingBox {
            center,
            half_width,
            half_height,
        }
    }
}