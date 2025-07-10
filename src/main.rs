use crate::types::rigid_body::RigidBody;
use crate::types::vector_2d::Vector2D;
use crate::utilities::world_functions::physics_step;
use crate::utilities::plotter::plot_trajectories;

mod types;
mod utilities;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut bodies: Vec<&mut RigidBody> = vec![];
    let mut body_1 = RigidBody::new(
        2.0,
        Vector2D::new(13.0, -5.0),
        Vector2D::new(1.0, 0.2),
        Vector2D::new(0.0, 0.0)
    );
    bodies.push(&mut body_1);

    let mut body_2 = RigidBody::new(
        15.0,
        Vector2D::new(4.0, 2.0),
        Vector2D::new(0.0, 0.2),
        Vector2D::new(0.0, 0.0)
    );
    bodies.push(&mut body_2);

    let mut body_3 = RigidBody::new(
        0.1,
        Vector2D::new(50.0, -60.0),
        Vector2D::new(15.0, -23.2),
        Vector2D::new(0.0, 0.0)
    );
    bodies.push(&mut body_3);

    let dt = 1.0 / 60.0;
    let mut body_positions: Vec<Vec<(f32, f32)>> = vec![Vec::new(); bodies.len()
    ];

    for _ in 0..300 {
        physics_step(&mut bodies, dt);

        for (i, body) in bodies.iter().enumerate() {
            body_positions[i].push((body.position.x, body.position.y));
        }
        // for (i, body) in bodies.iter_mut().enumerate() {
        //     println!("Frame: {}", frame);
        //     println!("Body {i}: \n{:#?}", body);
        //     println!();
        // }
    }
    plot_trajectories(&body_positions)?;

    Ok(())

}
