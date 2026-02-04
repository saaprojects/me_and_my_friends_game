use crate::core::GhostType;
use bevy::prelude::Resource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiritboxReply {
    Static,
    Here,
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

pub fn emf_five_candidate(
    ghost_type: GhostType,
    distance: f32,
    tuning: &EvidenceTuning,
) -> bool {
    ghost_type == GhostType::Spirit && distance <= overlap_distance(tuning)
}

pub fn spiritbox_reply(ghost_type: GhostType, overlap: bool) -> SpiritboxReply {
    match ghost_type {
        GhostType::Banshee => {
            if overlap {
                SpiritboxReply::Here
            } else {
                SpiritboxReply::Static
            }
        }
        GhostType::Spirit | GhostType::Onryo => SpiritboxReply::Static,
    }
}

pub fn spiritbox_is_evidence(reply: SpiritboxReply) -> bool {
    matches!(reply, SpiritboxReply::Here)
}

impl SpiritboxReply {
    pub fn as_str(self) -> &'static str {
        match self {
            SpiritboxReply::Static => "Static...",
            SpiritboxReply::Here => "Right... here.",
        }
    }
}
