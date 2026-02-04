use bevy::prelude::{Resource, Vec3};

use crate::core::GhostType;

#[derive(Clone)]
pub struct SpiritConfig {
    pub anchors: &'static [Vec3],
    pub watch_cos: f32,
    pub watch_distance: f32,
    pub grace_seconds: f32,
    pub rate_up: f32,
    pub rate_down: f32,
}

#[derive(Clone)]
pub struct BansheeConfig {
    pub anchors: &'static [Vec3],
    pub interact_distance: f32,
    pub timing_min: f32,
    pub timing_max: f32,
    pub fail_reset_seconds: f32,
}

impl BansheeConfig {
    pub fn sequence_len(&self) -> u8 {
        self.anchors.len() as u8
    }
}

#[derive(Clone)]
pub struct OnryoConfig {
    pub cursed_positions: &'static [Vec3],
    pub ritual_positions: &'static [Vec3],
    pub interact_distance: f32,
    pub carry_height: f32,
    pub stack_rate: f32,
    pub max_stacks: f32,
    pub stack_penalty_wrong: f32,
    pub stack_reward_correct: f32,
}

#[derive(Resource, Clone)]
pub struct ExorcismTables {
    pub spirit: SpiritConfig,
    pub banshee: BansheeConfig,
    pub onryo: OnryoConfig,
}

impl Default for ExorcismTables {
    fn default() -> Self {
        Self {
            spirit: SpiritConfig {
                anchors: &SPIRIT_ANCHORS,
                watch_cos: 0.85,
                watch_distance: 7.0,
                grace_seconds: 2.0,
                rate_up: 0.16,
                rate_down: 0.1,
            },
            banshee: BansheeConfig {
                anchors: &BANSHEE_ANCHORS,
                interact_distance: 1.6,
                timing_min: 0.6,
                timing_max: 3.5,
                fail_reset_seconds: 2.5,
            },
            onryo: OnryoConfig {
                cursed_positions: &ONRYO_CURSED,
                ritual_positions: &ONRYO_RITUALS,
                interact_distance: 1.8,
                carry_height: 1.1,
                stack_rate: 0.6,
                max_stacks: 5.0,
                stack_penalty_wrong: 1.0,
                stack_reward_correct: 2.0,
            },
        }
    }
}

pub fn puzzle_name(ghost_type: GhostType) -> &'static str {
    match ghost_type {
        GhostType::Spirit => "Spirit: The Vigil",
        GhostType::Banshee => "Banshee: The Lament",
        GhostType::Onryo => "Onryo: The Containment",
    }
}

const SPIRIT_ANCHORS: [Vec3; 3] = [
    Vec3::new(-6.0, 0.7, -6.0),
    Vec3::new(6.0, 0.7, -5.5),
    Vec3::new(-5.5, 0.7, 6.0),
];

const BANSHEE_ANCHORS: [Vec3; 3] = [
    Vec3::new(-4.0, 0.5, -2.0),
    Vec3::new(4.5, 0.5, -1.5),
    Vec3::new(0.0, 0.5, 5.0),
];

const ONRYO_CURSED: [Vec3; 3] = [
    Vec3::new(-6.5, 0.4, 0.0),
    Vec3::new(6.5, 0.4, 0.0),
    Vec3::new(0.0, 0.4, -6.5),
];

const ONRYO_RITUALS: [Vec3; 3] = [
    Vec3::new(-2.5, 0.1, 2.5),
    Vec3::new(2.5, 0.1, 2.5),
    Vec3::new(0.0, 0.1, 6.5),
];
