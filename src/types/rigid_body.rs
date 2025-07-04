use crate::types::vector_2d::Vector2D;

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub mass: f32,
    pub inv_mass: f32,
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub force: Vector2D,
}

impl RigidBody {
    pub fn new(mass: f32, position: Vector2D, velocity: Vector2D, force: Vector2D) -> Self {
        let inv_mass = if mass == 0.0 {
            0.0f32
        }
        else {
            1.0 / mass
        };

        RigidBody {
            mass,
            inv_mass,
            position,
            velocity,
            force,
        }
    }

    pub fn apply_force(&mut self, force: Vector2D) {
        self.force = self.force + force;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_new() {
        let body = RigidBody::new(1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0));
        assert_eq!(body.mass, 1.0);
        assert_eq!(body.inv_mass, 1.0);
        assert_eq!(body.position.x, 0.0);
        assert_eq!(body.position.y, 0.0);
        assert_eq!(body.velocity.x, 0.0);
        assert_eq!(body.velocity.y, 0.0);
        assert_eq!(body.force.x, 0.0);
    }

    #[test]
    fn test_rigid_body_new_zero_mass() {
        let body = RigidBody::new(0.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0));
        assert_eq!(body.mass, 0.0);
        assert_eq!(body.inv_mass, 0.0);
        assert_eq!(body.position.x, 0.0);
        assert_eq!(body.position.y, 0.0);
        assert_eq!(body.velocity.x, 0.0);
    }

    #[test]
    fn test_apply_force_basic() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );
        let force = Vector2D::new(1.0, 2.0);
        body.apply_force(force);
        assert_eq!(body.force.x, 1.0);
        assert_eq!(body.force.y, 2.0);
    }

    #[test]
    fn test_apply_force_multiple() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );
        body.apply_force(Vector2D::new(1.0, 2.0));
        body.apply_force(Vector2D::new(2.0, 3.0));
        assert_eq!(body.force.x, 3.0);
        assert_eq!(body.force.y, 5.0);
    }

    #[test]
    fn test_apply_force_zero() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0)
        );
        body.apply_force(Vector2D::new(0.0, 0.0));
        assert_eq!(body.force.x, 1.0);
        assert_eq!(body.force.y, 1.0);
    }

    #[test]
    fn test_apply_force_negative() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(1.0, 1.0)
        );
        body.apply_force(Vector2D::new(-2.0, -3.0));
        assert_eq!(body.force.x, -1.0);
        assert_eq!(body.force.y, -2.0);
    }

    #[test]
    fn test_apply_force_large_numbers() {
        let mut body = RigidBody::new(
            1.0,
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(0.0, 0.0)
        );
        body.apply_force(Vector2D::new(1000000.0, 2000000.0));
        assert_eq!(body.force.x, 1000000.0);
        assert_eq!(body.force.y, 2000000.0);
    }

}