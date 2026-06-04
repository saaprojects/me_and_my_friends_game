use crate::prelude::*;

#[derive(Resource)]
pub struct GhostState {
    pub position: Vec3,
}

#[derive(Resource, Default)]
pub struct GhostVelocity(pub Vec3);

#[derive(Resource, Default)]
pub struct GhostBobState {
    pub phase: f32,
    pub magnitude: f32,
}

#[derive(Component)]
pub struct GhostMarker;
