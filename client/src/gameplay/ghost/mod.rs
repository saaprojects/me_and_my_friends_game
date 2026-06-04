use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::{GhostBobState, GhostMarker, GhostState, GhostVelocity};

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::core::InputMap>()
            .init_resource::<crate::core::MovementConfig>()
            .init_resource::<GhostVelocity>()
            .init_resource::<GhostBobState>()
            .add_systems(
            Update,
            (systems::ghost_movement_system, systems::sync_ghost_marker),
        );
    }
}
