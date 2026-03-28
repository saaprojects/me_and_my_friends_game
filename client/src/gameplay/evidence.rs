use crate::core::GhostType;
use bevy::prelude::{Resource, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiritboxReply {
    Static,
    Here,
    Left,
    Right,
    Behind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiritboxBearing {
    Ahead,
    Left,
    Right,
    Behind,
}

#[derive(Resource, Clone)]
pub struct EvidenceTuning {
    pub tool_bubble_radius: f32,
    pub ghost_influence_radius: f32,
    pub emf_range_4: f32,
    pub emf_range_3: f32,
    pub emf_range_2: f32,
    pub emf_dwell_lock: f32,
    pub emf_dwell_decay_mul: f32,
    pub emf_facing_dot: f32,
    pub emf_smooth_rate: f32,
    pub emf_jitter_amp_23: f32,
    pub emf_jitter_amp_4: f32,
    pub emf_jitter_f1: f32,
    pub emf_jitter_f2: f32,
    pub emf_evidence_latch: f32,
    pub emf_jitter_phase: f32,
    pub spiritbox_range: f32,
    pub spiritbox_cooldown_hit: f32,
    pub spiritbox_cooldown_miss: f32,
}

impl Default for EvidenceTuning {
    fn default() -> Self {
        Self {
            tool_bubble_radius: 1.0,
            ghost_influence_radius: 1.0,
            emf_range_4: 2.5,
            emf_range_3: 4.5,
            emf_range_2: 6.5,
            emf_dwell_lock: 0.8,
            emf_dwell_decay_mul: 2.0,
            emf_facing_dot: 0.6,
            emf_smooth_rate: 10.0,
            emf_jitter_amp_23: 0.18,
            emf_jitter_amp_4: 0.35,
            emf_jitter_f1: 4.7,
            emf_jitter_f2: 9.1,
            emf_evidence_latch: 1.2,
            emf_jitter_phase: 0.0,
            spiritbox_range: 5.5,
            spiritbox_cooldown_hit: 1.6,
            spiritbox_cooldown_miss: 1.2,
        }
    }
}

pub fn overlap_distance(tuning: &EvidenceTuning) -> f32 {
    tuning.tool_bubble_radius + tuning.ghost_influence_radius
}

pub fn emf_level(
    ghost_type: GhostType,
    distance: f32,
    _same_room: bool,
    tuning: &EvidenceTuning,
) -> u8 {
    match ghost_type {
        GhostType::Spirit => {
            if distance <= tuning.emf_range_4 {
                4
            } else if distance <= tuning.emf_range_3 {
                3
            } else if distance <= tuning.emf_range_2 {
                2
            } else {
                1
            }
        }
        GhostType::Banshee | GhostType::Onryo => 1,
    }
}

pub fn emf_five_candidate(ghost_type: GhostType, distance: f32, tuning: &EvidenceTuning) -> bool {
    ghost_type == GhostType::Spirit && distance <= overlap_distance(tuning)
}

pub fn spiritbox_bearing(
    player_forward: Vec3,
    player_pos: Vec3,
    ghost_pos: Vec3,
) -> SpiritboxBearing {
    let forward_flat = Vec3::new(player_forward.x, 0.0, player_forward.z).normalize_or_zero();
    let to_ghost = ghost_pos - player_pos;
    let to_ghost_flat = Vec3::new(to_ghost.x, 0.0, to_ghost.z).normalize_or_zero();

    if forward_flat.length_squared() <= f32::EPSILON
        || to_ghost_flat.length_squared() <= f32::EPSILON
    {
        return SpiritboxBearing::Ahead;
    }

    let front_dot = forward_flat.dot(to_ghost_flat);
    if front_dot >= 0.35 {
        return SpiritboxBearing::Ahead;
    }
    if front_dot <= -0.35 {
        return SpiritboxBearing::Behind;
    }

    let right = Vec3::new(forward_flat.z, 0.0, -forward_flat.x).normalize_or_zero();
    if right.dot(to_ghost_flat) >= 0.0 {
        SpiritboxBearing::Right
    } else {
        SpiritboxBearing::Left
    }
}

pub fn spiritbox_reply(
    ghost_type: GhostType,
    same_room: bool,
    distance: f32,
    tuning: &EvidenceTuning,
    bearing: SpiritboxBearing,
) -> SpiritboxReply {
    if ghost_type != GhostType::Banshee || !same_room || distance > tuning.spiritbox_range {
        return SpiritboxReply::Static;
    }

    match bearing {
        SpiritboxBearing::Ahead => SpiritboxReply::Here,
        SpiritboxBearing::Left => SpiritboxReply::Left,
        SpiritboxBearing::Right => SpiritboxReply::Right,
        SpiritboxBearing::Behind => SpiritboxReply::Behind,
    }
}

pub fn spiritbox_is_evidence(reply: SpiritboxReply) -> bool {
    !matches!(reply, SpiritboxReply::Static)
}

impl SpiritboxReply {
    pub fn as_str(self) -> &'static str {
        match self {
            SpiritboxReply::Static => "Static...",
            SpiritboxReply::Here => "Right... here.",
            SpiritboxReply::Left => "To your left...",
            SpiritboxReply::Right => "To your right...",
            SpiritboxReply::Behind => "Behind you...",
        }
    }
}

#[cfg(test)]
#[path = "evidence_tests.rs"]
mod evidence_tests;
