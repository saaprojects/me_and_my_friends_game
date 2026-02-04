use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::{GhostMarker, GhostState};

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::ghost_movement_system, systems::sync_ghost_marker),
        );
    }
}
