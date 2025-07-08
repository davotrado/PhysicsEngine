use crate::types::rigid_body::RigidBody;
use crate::types::vector_2d::Vector2D;

pub fn integrate(rigid_body: &mut RigidBody, dt: f32) {
    if rigid_body.inv_mass == 0.0 {
        return;
    }

    let acceleration = rigid_body.force * rigid_body.inv_mass;
    rigid_body.velocity = rigid_body.velocity + acceleration * dt;
    rigid_body.position = rigid_body.position + rigid_body.velocity * dt;

    rigid_body.force = Vector2D::new(0.0, 0.0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrate_zero_mass() {
        let mut body = RigidBody::new(
            0.0,
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 2.0),
            Vector2D::new(3.0, 3.0)
        );
        let initial_position = body.position;
        let initial_velocity = body.velocity;

        integrate(&mut body, 1.0);

        assert_eq!(body.position.x, initial_position.x);
        assert_eq!(body.position.y, initial_position.y);
        assert_eq!(body.velocity.x, initial_velocity.x);
        assert_eq!(body.velocity.y, initial_velocity.y);
    }

    #[test]
    fn test_integrate_no_force() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 2.0),
            Vector2D::new(0.0, 0.0)
        );

        integrate(&mut body, 1.0);


        assert_eq!(body.velocity.x, 1.0);
        assert_eq!(body.velocity.y, 2.0);

        assert_eq!(body.position.x, 1.0);
        assert_eq!(body.position.y, 2.0);
    }

    #[test]
    fn test_integrate_with_force() {
        let mut body = RigidBody::new(
            2.0, // mass = 2.0, so inv_mass = 0.5
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(4.0, 6.0)
        );

        integrate(&mut body, 0.5);

        assert_eq!(body.velocity.x, 1.0);
        assert_eq!(body.velocity.y, 1.5);
        assert_eq!(body.position.x, 0.5);
        assert_eq!(body.position.y, 0.75);
        assert_eq!(body.force.x, 0.0);
        assert_eq!(body.force.y, 0.0);
    }

    #[test]
    fn test_integrate_force_reset() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0)
        );

        integrate(&mut body, 1.0);

        assert_eq!(body.force.x, 0.0);
        assert_eq!(body.force.y, 0.0);
    }
}
