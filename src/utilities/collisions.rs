use crate::types::circle::Circle;
use crate::types::bounding_box::BoundingBox;
use crate::types::rigid_body::RigidBody;
use crate::types::vector_2d::Vector2D;

pub fn circle_collision(circle_1: Circle, circle_2: Circle) -> bool {
    let delta = circle_1.center - circle_2.center;
    let squared_distance = delta.dot(&delta);
    let radius_sum = circle_1.radius + circle_2.radius;
    squared_distance <= radius_sum * radius_sum
}

pub fn bounding_box_collision(bounding_box_1: BoundingBox, bounding_box_2: BoundingBox) -> bool {

    ((bounding_box_1.center.x - bounding_box_2.center.x).abs() <=
        bounding_box_1.half_width + bounding_box_2.half_width) &
        ((bounding_box_1.center.y - bounding_box_2.center.y).abs() <=
        bounding_box_1.half_height + bounding_box_2.half_height)
}

pub fn resolve_collision(body_1: &mut RigidBody, body_2: &mut RigidBody, normal: Vector2D, overlap: f32) {
    let relative_velocity = body_2.velocity - body_1.velocity;
    let velocity_along_normal = relative_velocity.dot(&normal);

    if velocity_along_normal > 0.0 {
        return;
    }

    let restitution = 0.5;

    let mut impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
    impulse_scalar /= body_1.inv_mass + body_2.inv_mass;

    let impulse = normal * impulse_scalar;
    body_1.velocity = body_1.velocity - (impulse * body_1.inv_mass);
    body_2.velocity = body_2.velocity - (impulse * body_2.inv_mass);

    let percentage = 0.8;
    let allowable_overlap = 0.01;
    let overlap_correction = normal * (f32::max(overlap - allowable_overlap, 0.0) /
        (body_1.inv_mass + body_2.inv_mass) * percentage);
    body_1.position = body_1.position - overlap_correction * body_1.inv_mass;
    body_2.position = body_2.position + overlap_correction * body_2.inv_mass;
}
