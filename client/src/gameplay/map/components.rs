use crate::prelude::*;

#[derive(Resource)]
pub struct CollisionWorld {
    pub bounds: Bounds,
    pub obstacles: Vec<Obstacle>,
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}
