use crate::prelude::*;

use crate::core::{GhostTypeState, JournalState, MenuState, RoleState};
use crate::gameplay::evidence::{
    emf_five_candidate, emf_level, overlap_distance, spiritbox_is_evidence, spiritbox_reply,
    EvidenceTuning,
};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::investigator::Player;
use crate::gameplay::map::systems::room_id;

#[derive(Resource)]
pub struct EquipmentState {
    pub active: Equipment,
    pub emf_level: u8,
    pub emf_dwell: f32,
    pub emf_smoothed: f32,
    pub emf_evidence_latch: f32,
    pub spiritbox_message: String,
    pub spiritbox_cooldown: f32,
}

#[derive(Resource, Default)]
pub struct EvidenceState {
    pub emf_five: bool,
    pub spiritbox_response: bool,
}

pub fn handle_equipment_input(
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    journal: Res<JournalState>,
    mut equipment: ResMut<EquipmentState>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        equipment.active = Equipment::Emf;
    }
    if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        equipment.active = Equipment::Spiritbox;
    }
}

pub fn update_emf_reading(
    time: Res<Time>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    ghost: Res<GhostState>,
    control: Res<CameraControl>,
    tuning: Res<EvidenceTuning>,
    mut equipment: ResMut<EquipmentState>,
    ghost_type: Res<GhostTypeState>,
    mut evidence: ResMut<EvidenceState>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<Camera>>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        equipment.emf_level = 0;
        equipment.emf_dwell = 0.0;
        equipment.emf_evidence_latch = 0.0;
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        equipment.emf_level = 0;
        equipment.emf_dwell = 0.0;
        equipment.emf_evidence_latch = 0.0;
        return;
    };

    let to_ghost = ghost.position - player_transform.translation;
    let distance = to_ghost.length();
    let facing = facing_ghost(
        player_transform.translation,
        ghost.position,
        &control,
        camera.get_single().ok(),
        tuning.emf_facing_dot,
    );

    let same_room = room_id(player_transform.translation) == room_id(ghost.position);
    let base_level = emf_level(ghost_type.active, distance, same_room, &tuning);
    let overlaps = distance <= overlap_distance(&tuning);
    let candidate_five =
        overlaps && emf_five_candidate(ghost_type.active, distance, &tuning) && facing;

    let dt = time.delta_seconds();
    let dwell_lock = tuning.emf_dwell_lock;
    if candidate_five {
        equipment.emf_dwell = (equipment.emf_dwell + dt).min(dwell_lock);
    } else {
        equipment.emf_dwell = (equipment.emf_dwell - dt * tuning.emf_dwell_decay_mul).max(0.0);
    }

    let locked_five = equipment.emf_dwell >= dwell_lock;
    if locked_five {
        equipment.emf_evidence_latch = tuning.emf_evidence_latch;
    } else {
        equipment.emf_evidence_latch = (equipment.emf_evidence_latch - dt).max(0.0);
    }

    let mut target_level = if locked_five { 5.0 } else { base_level as f32 };
    if !facing {
        target_level = target_level.min(4.0);
    }

    if !locked_five {
        let jitter_amp = match base_level {
            2 | 3 => tuning.emf_jitter_amp_23,
            4 => tuning.emf_jitter_amp_4,
            _ => 0.0,
        };
        if jitter_amp > 0.0 {
            let t = time.elapsed_seconds() + tuning.emf_jitter_phase;
            let jitter = (t * tuning.emf_jitter_f1).sin() * 0.6
                + (t * tuning.emf_jitter_f2).sin() * 0.4;
            target_level += jitter * jitter_amp;
        }
        target_level = target_level.clamp(0.0, 4.49);
    }

    let smooth_rate = tuning.emf_smooth_rate;
    let alpha = 1.0 - (-smooth_rate * dt).exp();
    equipment.emf_smoothed += (target_level - equipment.emf_smoothed) * alpha;
    equipment.emf_level = equipment.emf_smoothed.round().clamp(0.0, 5.0) as u8;

    if equipment.active == Equipment::Emf && equipment.emf_evidence_latch > 0.0 {
        evidence.emf_five = true;
    }
}

pub fn handle_spiritbox(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    ghost: Res<GhostState>,
    tuning: Res<EvidenceTuning>,
    mut equipment: ResMut<EquipmentState>,
    ghost_type: Res<GhostTypeState>,
    mut evidence: ResMut<EvidenceState>,
    player: Query<&Transform, With<Player>>,
) {
    if equipment.spiritbox_cooldown > 0.0 {
        equipment.spiritbox_cooldown =
            (equipment.spiritbox_cooldown - time.delta_seconds()).max(0.0);
    }

    if menu.open
        || journal.open
        || role.current != Role::Investigator
        || equipment.active != Equipment::Spiritbox
    {
        return;
    }

    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    if equipment.spiritbox_cooldown > 0.0 {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        equipment.spiritbox_message = "Only static...".into();
        equipment.spiritbox_cooldown = tuning.spiritbox_cooldown_miss;
        return;
    };

    let distance = player_transform.translation.distance(ghost.position);
    let overlaps = distance <= overlap_distance(&tuning);
    let reply = spiritbox_reply(ghost_type.active, overlaps);
    equipment.spiritbox_message = reply.as_str().to_string();
    let is_evidence = spiritbox_is_evidence(reply);
    if is_evidence {
        evidence.spiritbox_response = true;
    }
    equipment.spiritbox_cooldown = if is_evidence {
        tuning.spiritbox_cooldown_hit
    } else {
        tuning.spiritbox_cooldown_miss
    };
}

fn facing_ghost(
    player_pos: Vec3,
    ghost_pos: Vec3,
    control: &CameraControl,
    camera: Option<&Transform>,
    facing_dot: f32,
) -> bool {
    let to_ghost = ghost_pos - player_pos;
    let to_ghost_flat = Vec3::new(to_ghost.x, 0.0, to_ghost.z);
    if to_ghost_flat.length_squared() <= f32::EPSILON {
        return true;
    }
    if let Some(cam_transform) = camera {
        let forward = cam_transform.forward();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let dir = to_ghost_flat.normalize_or_zero();
        return forward_flat.dot(dir) >= facing_dot;
    }
    let yaw = control.yaw + std::f32::consts::PI;
    let forward = Vec3::new(yaw.sin(), 0.0, yaw.cos()).normalize_or_zero();
    let dir = to_ghost_flat.normalize_or_zero();
    forward.dot(dir) >= facing_dot
}
