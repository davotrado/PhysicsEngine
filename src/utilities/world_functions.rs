use crate::types::rigid_body::RigidBody;
use crate::utilities::constants::G;
use crate::utilities::math_operations::integrate;

pub fn physics_step(bodies: &mut Vec<&mut RigidBody>, dt: f32) {
    for body in bodies {
        body.apply_force(G * body.mass);
        integrate(body, dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::vector_2d::Vector2D;

    #[test]
    fn test_physics_step_single_body() {
        let mut body = RigidBody::new(
            2.0, // mass = 2.0
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );

        physics_step(&mut vec![&mut body], 1.0);

        assert_eq!(body.velocity.x, 0.0);
        assert!(f32::abs(body.velocity.y - 9.81) < 0.001);
        assert_eq!(body.position.x, 0.0);
        assert!(f32::abs(body.position.y - 9.81) < 0.001);
    }

    #[test]
    fn test_physics_step_multiple_bodies() {
        let mut body1 = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );

        let mut body2 = RigidBody::new(
            2.0,
            Vector2D::new(1.0, 1.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );

        physics_step(&mut vec![&mut body1, &mut body2], 1.0);

        assert_eq!(body1.position.x, 0.0);
        assert!(f32::abs(body1.position.y - 9.81) < 0.001);

        assert_eq!(body2.position.x, 1.0);
        assert!(f32::abs(body2.position.y - (1.0 + 9.81)) < 0.001);
    }

    #[test]
    fn test_physics_step_zero_mass_body() {
        let mut body = RigidBody::new(
            0.0,
            Vector2D::new(1.0, 1.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );

        let initial_position = body.position;
        physics_step(&mut vec![&mut body], 1.0);
        assert_eq!(body.position.x, initial_position.x);
        assert_eq!(body.position.y, initial_position.y);
    }

    #[test]
    fn test_physics_step_small_timestep() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );

        physics_step(&mut vec![&mut body], 0.5);
        assert_eq!(body.velocity.x, 0.0);
        assert!(f32::abs(body.velocity.y - 4.905) < 0.001);
        assert_eq!(body.position.x, 0.0);
        assert!(f32::abs(body.position.y - 2.4525) < 0.001);
    }

    #[test]
    fn test_physics_step_empty_bodies() {
        let mut empty_vec:  Vec<&mut RigidBody> = vec![];
        physics_step(&mut empty_vec, 1.0);
    }
}
