use crate::prelude::*;

#[derive(Resource)]
pub struct GhostState {
    pub position: Vec3,
}

#[derive(Component)]
pub struct GhostMarker;
