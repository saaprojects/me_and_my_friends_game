use bevy::prelude::{KeyCode, Resource};
use std::time::{SystemTime, UNIX_EPOCH};

/// Deterministic-when-seeded RNG. Default seeds from wall-clock time.
/// In tests, construct `GameRng { state: <fixed_value> }` for reproducibility.
#[derive(Resource)]
pub struct GameRng {
    pub state: u64,
}

impl GameRng {
    pub fn next_seed(&mut self, salt: u64) -> u64 {
        self.state = self.state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let mut seed = self.state ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        if seed == 0 {
            seed = 0xC2B2_AE35_79B9_83EF;
        }
        seed
    }
}

impl Default for GameRng {
    fn default() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        Self {
            state: nanos.wrapping_add(0xC2B2_AE35_79B9_83EF),
        }
    }
}

#[derive(Resource, Clone)]
pub struct InputMap {
    pub move_forward: KeyCode,
    pub move_forward_alt: KeyCode,
    pub move_back: KeyCode,
    pub move_back_alt: KeyCode,
    pub move_left: KeyCode,
    pub move_left_alt: KeyCode,
    pub move_right: KeyCode,
    pub move_right_alt: KeyCode,
    pub sprint: KeyCode,
    pub sprint_alt: KeyCode,
    pub role_toggle: KeyCode,
    pub role_toggle_alt: KeyCode,
    pub toggle_lights: KeyCode,
    pub interact: KeyCode,
    pub spiritbox_ask: KeyCode,
    pub menu_toggle: KeyCode,
    pub journal_toggle: KeyCode,
    pub tool_emf: KeyCode,
    pub tool_emf_alt: KeyCode,
    pub tool_spiritbox: KeyCode,
    pub tool_spiritbox_alt: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_forward_alt: KeyCode::ArrowUp,
            move_back: KeyCode::KeyS,
            move_back_alt: KeyCode::ArrowDown,
            move_left: KeyCode::KeyA,
            move_left_alt: KeyCode::ArrowLeft,
            move_right: KeyCode::KeyD,
            move_right_alt: KeyCode::ArrowRight,
            sprint: KeyCode::ShiftLeft,
            sprint_alt: KeyCode::ShiftRight,
            role_toggle: KeyCode::Tab,
            role_toggle_alt: KeyCode::KeyT,
            toggle_lights: KeyCode::KeyL,
            interact: KeyCode::KeyF,
            spiritbox_ask: KeyCode::KeyE,
            menu_toggle: KeyCode::Escape,
            journal_toggle: KeyCode::KeyJ,
            tool_emf: KeyCode::Digit1,
            tool_emf_alt: KeyCode::Numpad1,
            tool_spiritbox: KeyCode::Digit2,
            tool_spiritbox_alt: KeyCode::Numpad2,
        }
    }
}

#[derive(Resource, Clone)]
pub struct MovementConfig {
    pub ghost_speed: f32,
    pub ghost_sprint_mul: f32,
    pub investigator_speed: f32,
    pub investigator_sprint_mul: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            ghost_speed: 5.2,
            ghost_sprint_mul: 1.6,
            investigator_speed: 3.6,
            investigator_sprint_mul: 1.6,
        }
    }
}

#[derive(Resource, Clone)]
pub struct CameraConfig {
    pub radius: f32,
    pub min_distance: f32,
    pub smooth_rate: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            radius: 4.8,
            min_distance: 1.2,
            smooth_rate: 10.0,
        }
    }
}
