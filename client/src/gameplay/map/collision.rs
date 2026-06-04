use bevy::prelude::Vec3;

use super::components::{Bounds, Obstacle};

pub fn clamp_to_bounds(pos: &mut Vec3, bounds: Bounds, radius: f32) {
    pos.x = pos.x.clamp(bounds.min_x + radius, bounds.max_x - radius);
    pos.z = pos.z.clamp(bounds.min_z + radius, bounds.max_z - radius);
}

pub fn collides(pos: Vec3, radius: f32, obstacles: &[Obstacle]) -> bool {
    obstacles.iter().any(|obs| {
        let hit_x = pos.x + radius > obs.min_x && pos.x - radius < obs.max_x;
        let hit_z = pos.z + radius > obs.min_z && pos.z - radius < obs.max_z;
        hit_x && hit_z
    })
}

pub fn move_with_collisions(
    pos: &mut Vec3,
    movement: Vec3,
    radius: f32,
    bounds: Bounds,
    obstacles: &[Obstacle],
    block_interior: bool,
) {
    let mut next = *pos + movement;
    clamp_to_bounds(&mut next, bounds, radius);

    if !block_interior {
        *pos = next;
        return;
    }

    if !collides(next, radius, obstacles) {
        *pos = next;
        return;
    }

    let mut try_x = *pos;
    try_x.x = next.x;
    clamp_to_bounds(&mut try_x, bounds, radius);
    if !collides(try_x, radius, obstacles) {
        *pos = try_x;
        return;
    }

    let mut try_z = *pos;
    try_z.z = next.z;
    clamp_to_bounds(&mut try_z, bounds, radius);
    if !collides(try_z, radius, obstacles) {
        *pos = try_z;
    }
}
