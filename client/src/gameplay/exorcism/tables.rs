use bevy::prelude::Resource;

use crate::core::GhostType;

#[derive(Clone)]
pub struct SpiritConfig {
    pub watch_cos: f32,
    pub watch_distance: f32,
    pub grace_seconds: f32,
    pub rate_up: f32,
    pub rate_down: f32,
}

#[derive(Clone)]
pub struct BansheeConfig {
    pub default_sequence_len: u8,
    pub interact_distance: f32,
    pub timing_min: f32,
    pub timing_max: f32,
    pub fail_reset_seconds: f32,
}

impl BansheeConfig {
    pub fn sequence_len(&self) -> u8 {
        self.default_sequence_len
    }
}

#[derive(Clone)]
pub struct OnryoConfig {
    pub default_ritual_count: u8,
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
                watch_cos: 0.75,
                watch_distance: 9.0,
                grace_seconds: 4.5,
                rate_up: 0.16,
                rate_down: 0.1,
            },
            banshee: BansheeConfig {
                default_sequence_len: 3,
                interact_distance: 1.6,
                timing_min: 0.6,
                timing_max: 3.5,
                fail_reset_seconds: 2.5,
            },
            onryo: OnryoConfig {
                default_ritual_count: 3,
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
