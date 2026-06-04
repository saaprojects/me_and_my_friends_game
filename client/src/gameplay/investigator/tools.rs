use crate::prelude::*;

use crate::core::{GhostTypeState, InputMap, JournalState, MenuState, RoleState, SessionState};
use crate::gameplay::evidence::{
    spiritbox_bearing, spiritbox_is_evidence, spiritbox_reply, tick_emf, EvidenceTuning,
};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::investigator::Player;
use crate::gameplay::map::systems::{room_id, room_id_in_house};
use crate::gameplay::map::HouseLayout;

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

impl Default for EquipmentState {
    fn default() -> Self {
        Self {
            active: Equipment::Emf,
            emf_level: 0,
            emf_dwell: 0.0,
            emf_smoothed: 0.0,
            emf_evidence_latch: 0.0,
            spiritbox_message: "Silence...".to_string(),
            spiritbox_cooldown: 0.0,
        }
    }
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
    input: Res<InputMap>,
    mut equipment: ResMut<EquipmentState>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    if keys.just_pressed(input.tool_emf) || keys.just_pressed(input.tool_emf_alt) {
        equipment.active = Equipment::Emf;
    }
    if keys.just_pressed(input.tool_spiritbox) || keys.just_pressed(input.tool_spiritbox_alt) {
        equipment.active = Equipment::Spiritbox;
    }
}

pub fn update_emf_reading(
    time: Res<Time>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    session: Res<SessionState>,
    ghost: Res<GhostState>,
    control: Res<CameraControl>,
    tuning: Res<EvidenceTuning>,
    mut equipment: ResMut<EquipmentState>,
    ghost_type: Res<GhostTypeState>,
    mut evidence: ResMut<EvidenceState>,
    house_layout: Option<Res<HouseLayout>>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<Camera>>,
) {
    if menu.open || journal.open || role.current != Role::Investigator || !session.started {
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

    let distance = ghost.position.distance(player_transform.translation);
    let facing = facing_ghost(
        player_transform.translation,
        ghost.position,
        view_forward(&control, camera.get_single().ok()),
        tuning.emf_facing_dot,
    );
    let player_room = house_layout
        .as_ref()
        .and_then(|layout| room_id_in_house(layout, player_transform.translation))
        .unwrap_or_else(|| room_id(player_transform.translation));
    let ghost_room = house_layout
        .as_ref()
        .and_then(|layout| room_id_in_house(layout, ghost.position))
        .unwrap_or_else(|| room_id(ghost.position));
    let same_room = player_room == ghost_room;

    let out = tick_emf(
        distance,
        facing,
        ghost_type.active,
        same_room,
        equipment.emf_dwell,
        equipment.emf_smoothed,
        equipment.emf_evidence_latch,
        equipment.active,
        time.delta_seconds(),
        time.elapsed_seconds(),
        &tuning,
    );
    equipment.emf_dwell = out.emf_dwell;
    equipment.emf_evidence_latch = out.emf_evidence_latch;
    equipment.emf_smoothed = out.emf_smoothed;
    equipment.emf_level = out.emf_level;
    if out.trigger_evidence {
        evidence.emf_five = true;
    }
}

pub fn handle_spiritbox(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    session: Res<SessionState>,
    input: Res<InputMap>,
    ghost: Res<GhostState>,
    control: Res<CameraControl>,
    tuning: Res<EvidenceTuning>,
    mut equipment: ResMut<EquipmentState>,
    ghost_type: Res<GhostTypeState>,
    mut evidence: ResMut<EvidenceState>,
    house_layout: Option<Res<HouseLayout>>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<Camera>>,
) {
    if equipment.spiritbox_cooldown > 0.0 {
        equipment.spiritbox_cooldown =
            (equipment.spiritbox_cooldown - time.delta_seconds()).max(0.0);
    }

    if menu.open
        || journal.open
        || role.current != Role::Investigator
        || !session.started
        || equipment.active != Equipment::Spiritbox
    {
        return;
    }

    if !keys.just_pressed(input.spiritbox_ask) {
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
    let player_room = house_layout
        .as_ref()
        .and_then(|layout| room_id_in_house(layout, player_transform.translation))
        .unwrap_or_else(|| room_id(player_transform.translation));
    let ghost_room = house_layout
        .as_ref()
        .and_then(|layout| room_id_in_house(layout, ghost.position))
        .unwrap_or_else(|| room_id(ghost.position));
    let same_room = player_room == ghost_room;
    let bearing = spiritbox_bearing(
        view_forward(&control, camera.get_single().ok()),
        player_transform.translation,
        ghost.position,
    );
    let reply = spiritbox_reply(ghost_type.active, same_room, distance, &tuning, bearing);
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

fn facing_ghost(player_pos: Vec3, ghost_pos: Vec3, forward: Vec3, facing_dot: f32) -> bool {
    let to_ghost = ghost_pos - player_pos;
    let to_ghost_flat = Vec3::new(to_ghost.x, 0.0, to_ghost.z);
    if to_ghost_flat.length_squared() <= f32::EPSILON {
        return true;
    }
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let dir = to_ghost_flat.normalize_or_zero();
    forward_flat.dot(dir) >= facing_dot
}

fn view_forward(control: &CameraControl, camera: Option<&Transform>) -> Vec3 {
    if let Some(cam_transform) = camera {
        let forward = cam_transform.forward();
        return Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    }

    let yaw = control.yaw + std::f32::consts::PI;
    Vec3::new(yaw.sin(), 0.0, yaw.cos()).normalize_or_zero()
}
