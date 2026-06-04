use bevy::prelude::Vec3;

use crate::core::GameRng;
use super::components::HouseLayout;

pub fn room_id(position: Vec3) -> u8 {
    let x = if position.x >= 0.0 { 1 } else { 0 };
    let z = if position.z >= 0.0 { 1 } else { 0 };
    (x << 1) | z
}

pub fn room_id_in_house(layout: &HouseLayout, position: Vec3) -> Option<u8> {
    layout
        .rooms
        .iter()
        .find(|room| room.bounds.contains_xz(position))
        .map(|room| room.id)
}

pub fn default_house_layout() -> HouseLayout {
    HouseLayout::two_room()
}

pub fn random_round_start_positions() -> (Vec3, Vec3) {
    default_house_layout().random_start_positions(&mut GameRng::default())
}

pub fn shortest_angle(current: f32, target: f32) -> f32 {
    let mut diff = target - current;
    while diff > std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }
    diff
}

#[allow(dead_code)]
pub fn investigator_spawn_position() -> Vec3 {
    default_house_layout().investigator_spawn
}

#[allow(dead_code)]
pub fn ghost_spawn_positions() -> Vec<Vec3> {
    default_house_layout().ghost_spawns
}

#[cfg(test)]
pub fn investigator_spawn_positions() -> Vec<Vec3> {
    default_house_layout().investigator_spawn_candidates()
}

#[cfg(test)]
pub fn random_investigator_spawn_position() -> Vec3 {
    default_house_layout().random_investigator_spawn(&mut GameRng::default())
}

#[cfg(test)]
pub fn random_ghost_spawn_position() -> Vec3 {
    default_house_layout().random_ghost_spawn(&mut GameRng::default())
}
