use crate::prelude::*;

use crate::core::{GhostType, GhostTypeState, JournalState, MenuState, Role, RoleState};
use crate::gameplay::exorcism::tables::ExorcismTables;
use crate::gameplay::exorcism::{ExorcismState, ExorcismStatus, RoomLights};
use super::rules;

#[derive(Component)]
pub struct SpiritAnchor {
    pub room_id: u8,
    pub last_seen: f32,
}

#[derive(Component)]
pub struct SpiritMarker;

#[derive(Resource)]
pub struct SpiritPuzzle {
    pub progress: f32,
}

impl Default for SpiritPuzzle {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}

pub fn update_spirit_puzzle(
    time: Res<Time>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<super::InvestigationState>,
    tables: Res<ExorcismTables>,
    journal: Res<JournalState>,
    lights: Res<RoomLights>,
    mut anchors: Query<(&mut SpiritAnchor, &Transform)>,
    camera: Query<&Transform, With<Camera>>,
    mut spirit: ResMut<SpiritPuzzle>,
    mut status: ResMut<ExorcismStatus>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    let Some(puzzle_type) = investigation.guess else {
        return;
    };
    if !investigation.confirmed || puzzle_type != GhostType::Spirit {
        return;
    }

    let Ok(camera_transform) = camera.get_single() else {
        return;
    };
    let cam_pos = camera_transform.translation;
    let cam_forward = camera_transform.forward();
    let watch_cos = tables.spirit.watch_cos;
    let max_distance = tables.spirit.watch_distance;
    let grace_seconds = tables.spirit.grace_seconds;

    let mut recent_count = 0u8;
    let mut total_count = 0u8;
    for (mut anchor, transform) in anchors.iter_mut() {
        total_count = total_count.saturating_add(1);
        let to_anchor = transform.translation - cam_pos;
        let distance = to_anchor.length();
        let dir = to_anchor.normalize_or_zero();
        let seen = distance <= max_distance
            && cam_forward.dot(dir) >= watch_cos
            && lights.is_enabled(anchor.room_id);
        if seen {
            anchor.last_seen = 0.0;
        } else {
            anchor.last_seen += time.delta_seconds();
        }
        if anchor.last_seen <= grace_seconds {
            recent_count = recent_count.saturating_add(1);
        }
    }

    let required_count = total_count.clamp(1, 2);
    let target_progress = if required_count == 0 {
        0.0
    } else {
        (recent_count as f32 / required_count as f32).clamp(0.0, 1.0)
    };

    spirit.progress = rules::spirit_progress(
        spirit.progress,
        target_progress,
        time.delta_seconds(),
        tables.spirit.rate_up,
        tables.spirit.rate_down,
    );
    status.progress = spirit.progress;
    status.stage = recent_count;
    status.stacks = 0.0;
    status.max_stacks = required_count as f32;
    if spirit.progress >= 1.0 {
        if ghost_type.active == GhostType::Spirit {
            status.state = ExorcismState::Complete;
        } else {
            status.state = ExorcismState::Failed;
        }
    } else {
        status.state = ExorcismState::Progress(spirit.progress);
    }
}
