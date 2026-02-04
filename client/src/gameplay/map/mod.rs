use crate::prelude::*;

pub mod components;
pub mod systems;

pub use components::CollisionWorld;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionWorld {
            bounds: components::Bounds {
                min_x: -9.4,
                max_x: 9.4,
                min_z: -9.4,
                max_z: 9.4,
            },
            obstacles: vec![
                components::Obstacle {
                    min_x: -4.6,
                    max_x: -2.4,
                    min_z: -1.5,
                    max_z: -0.5,
                },
                components::Obstacle {
                    min_x: 3.3,
                    max_x: 4.7,
                    min_z: 2.3,
                    max_z: 3.7,
                },
            ],
        })
        .add_systems(Startup, systems::setup_scene);
    }
}
