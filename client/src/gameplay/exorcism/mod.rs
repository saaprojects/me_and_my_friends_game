use crate::prelude::*;

use crate::core::{GhostType, GhostTypeState, JournalState, MenuState, Role, RoleState};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::map::systems::room_id;

pub mod rules;
pub mod tables;

use tables::ExorcismTables;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExorcismState {
    Inactive,
    Stage(u8),
    Progress(f32),
    Failed,
    Complete,
}

#[derive(Resource)]
pub struct ExorcismStatus {
    pub state: ExorcismState,
    pub progress: f32,
    pub stage: u8,
    pub stacks: f32,
    pub max_stacks: f32,
}

#[derive(Resource, Default)]
pub struct InvestigationState {
    pub guess: Option<GhostType>,
    pub confirmed: bool,
}

#[derive(Resource)]
pub struct PuzzleSpawned(pub bool);

#[derive(Resource)]
struct RoomLights {
    rooms: [bool; 4],
}

#[derive(Component)]
struct PuzzleEntity;

#[derive(Component)]
struct SpiritAnchor {
    room_id: u8,
    last_seen: f32,
}

#[derive(Component)]
pub struct SpiritMarker;

#[derive(Component)]
struct BansheeAnchor {
    index: u8,
}

#[derive(Component)]
struct OnryoCursed {
    placed: bool,
}

#[derive(Component)]
struct OnryoRitual {
    index: u8,
}

#[derive(Resource)]
struct SpiritPuzzle {
    progress: f32,
}

#[derive(Resource)]
struct BansheePuzzle {
    stage: u8,
    time_since_trigger: f32,
    failed_timer: f32,
}

#[derive(Resource)]
struct OnryoPuzzle {
    stage: u8,
    stacks: f32,
    max_stacks: f32,
    carrying: Option<Entity>,
}

pub struct ExorcismPlugin;

impl Plugin for ExorcismPlugin {
    fn build(&self, app: &mut App) {
        let tables = ExorcismTables::default();
        app.insert_resource(tables.clone())
            .insert_resource(PuzzleSpawned(false))
            .insert_resource(RoomLights { rooms: [true; 4] })
            .insert_resource(SpiritPuzzle { progress: 0.0 })
            .insert_resource(BansheePuzzle {
                stage: 0,
                time_since_trigger: 0.0,
                failed_timer: 0.0,
            })
            .insert_resource(OnryoPuzzle {
                stage: 0,
                stacks: 0.0,
                max_stacks: tables.onryo.max_stacks,
                carrying: None,
            })
            .insert_resource(ExorcismStatus {
                state: ExorcismState::Inactive,
                progress: 0.0,
                stage: 0,
                stacks: 0.0,
                max_stacks: tables.onryo.max_stacks,
            })
            .insert_resource(InvestigationState::default())
            .add_systems(
                Update,
                (
                    maybe_reset_puzzle,
                    spawn_puzzle_entities,
                    ghost_toggle_lights,
                    update_spirit_puzzle,
                    update_banshee_puzzle,
                    update_onryo_puzzle,
                ),
            );
    }
}

fn maybe_reset_puzzle(
    spawned: Res<PuzzleSpawned>,
    tables: Res<ExorcismTables>,
    mut status: ResMut<ExorcismStatus>,
    mut spirit: ResMut<SpiritPuzzle>,
    mut banshee: ResMut<BansheePuzzle>,
    mut onryo: ResMut<OnryoPuzzle>,
    mut lights: ResMut<RoomLights>,
    mut anchors: Query<Entity, With<PuzzleEntity>>,
    mut commands: Commands,
    menu: Res<MenuState>,
    investigation: Res<InvestigationState>,
) {
    if menu.open {
        return;
    }
    if spawned.0 {
        return;
    }

    for entity in anchors.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    status.state = ExorcismState::Inactive;
    status.progress = 0.0;
    status.stage = 0;
    status.stacks = 0.0;
    onryo.max_stacks = tables.onryo.max_stacks;
    status.max_stacks = tables.onryo.max_stacks;

    spirit.progress = 0.0;
    banshee.stage = 0;
    banshee.time_since_trigger = 0.0;
    banshee.failed_timer = 0.0;
    onryo.stage = 0;
    onryo.stacks = 0.0;
    onryo.carrying = None;
    lights.rooms = [true; 4];

    if !investigation.confirmed {
        status.state = ExorcismState::Inactive;
        status.progress = 0.0;
        status.stage = 0;
        status.stacks = 0.0;
        status.max_stacks = 0.0;
    }
}

fn spawn_puzzle_entities(
    mut commands: Commands,
    mut spawned: ResMut<PuzzleSpawned>,
    investigation: Res<InvestigationState>,
    menu: Res<MenuState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tables: Res<ExorcismTables>,
) {
    if spawned.0 || menu.open || !investigation.confirmed {
        return;
    }

    let Some(puzzle_type) = investigation.guess else {
        return;
    };

    match puzzle_type {
        crate::core::GhostType::Spirit => {
            let anchor_mesh = meshes.add(Cuboid::new(0.35, 1.4, 0.1));
            let anchor_material = materials.add(Color::srgb(0.7, 0.75, 0.9));
            let marker_mesh = meshes.add(Cylinder::new(0.6, 0.04));
            let marker_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.25, 0.55, 0.95),
                emissive: Color::srgb(0.12, 0.22, 0.45).into(),
                ..default()
            });
            for pos in tables.spirit.anchors.iter().copied() {
                commands.spawn((
                    PbrBundle {
                        mesh: anchor_mesh.clone(),
                        material: anchor_material.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    SpiritAnchor {
                        room_id: room_id(pos),
                        last_seen: 999.0,
                    },
                    PuzzleEntity,
                ));
                commands.spawn((
                    PbrBundle {
                        mesh: marker_mesh.clone(),
                        material: marker_material.clone(),
                        transform: Transform::from_translation(Vec3::new(pos.x, 0.02, pos.z)),
                        ..default()
                    },
                    SpiritMarker,
                    PuzzleEntity,
                ));
            }
        }
        crate::core::GhostType::Banshee => {
            let anchor_mesh = meshes.add(Sphere::new(0.3).mesh().uv(16, 12));
            let anchor_material = materials.add(Color::srgb(0.8, 0.4, 0.9));
            for (index, pos) in tables.banshee.anchors.iter().enumerate() {
                commands.spawn((
                    PbrBundle {
                        mesh: anchor_mesh.clone(),
                        material: anchor_material.clone(),
                        transform: Transform::from_translation(*pos),
                        ..default()
                    },
                    BansheeAnchor {
                        index: index as u8,
                    },
                    PuzzleEntity,
                ));
            }
        }
        crate::core::GhostType::Onryo => {
            let cursed_mesh = meshes.add(Sphere::new(0.25).mesh().uv(16, 12));
            let cursed_material = materials.add(Color::srgb(0.9, 0.35, 0.35));
            let ritual_mesh = meshes.add(Cuboid::new(0.9, 0.05, 0.9));
            let ritual_material = materials.add(Color::srgb(0.35, 0.6, 0.9));

            for pos in tables.onryo.cursed_positions.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: cursed_mesh.clone(),
                        material: cursed_material.clone(),
                        transform: Transform::from_translation(*pos),
                        ..default()
                    },
                    OnryoCursed { placed: false },
                    PuzzleEntity,
                ));
            }

            for (index, pos) in tables.onryo.ritual_positions.iter().enumerate() {
                commands.spawn((
                    PbrBundle {
                        mesh: ritual_mesh.clone(),
                        material: ritual_material.clone(),
                        transform: Transform::from_translation(*pos),
                        ..default()
                    },
                    OnryoRitual {
                        index: index as u8,
                    },
                    PuzzleEntity,
                ));
            }
        }
    }

    spawned.0 = true;
}

fn ghost_toggle_lights(
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost: Res<GhostState>,
    mut lights: ResMut<RoomLights>,
) {
    if menu.open || role.current != Role::Ghost {
        return;
    }
    if keys.just_pressed(KeyCode::KeyL) {
        let room = room_id(ghost.position);
        if let Some(slot) = lights.rooms.get_mut(room as usize) {
            *slot = !*slot;
        }
    }
}

fn update_spirit_puzzle(
    time: Res<Time>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<InvestigationState>,
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

    let mut all_watched = true;
    for (mut anchor, transform) in anchors.iter_mut() {
        let to_anchor = transform.translation - cam_pos;
        let distance = to_anchor.length();
        let dir = to_anchor.normalize_or_zero();
        let seen = distance <= max_distance
            && cam_forward.dot(dir) >= watch_cos
            && *lights.rooms.get(anchor.room_id as usize).unwrap_or(&true);
        if seen {
            anchor.last_seen = 0.0;
        } else {
            anchor.last_seen += time.delta_seconds();
        }
        if anchor.last_seen > grace_seconds {
            all_watched = false;
        }
    }

    spirit.progress = rules::spirit_progress(
        spirit.progress,
        all_watched,
        time.delta_seconds(),
        tables.spirit.rate_up,
        tables.spirit.rate_down,
    );
    status.progress = spirit.progress;
    status.stage = 0;
    status.stacks = 0.0;
    status.max_stacks = 0.0;
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

fn update_banshee_puzzle(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<InvestigationState>,
    tables: Res<ExorcismTables>,
    journal: Res<JournalState>,
    player: Query<&Transform, With<crate::gameplay::investigator::Player>>,
    anchors: Query<(&BansheeAnchor, &Transform)>,
    mut banshee: ResMut<BansheePuzzle>,
    mut status: ResMut<ExorcismStatus>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    let Some(puzzle_type) = investigation.guess else {
        return;
    };
    if !investigation.confirmed || puzzle_type != GhostType::Banshee {
        return;
    }

    let sequence_len = tables.banshee.sequence_len();
    banshee.time_since_trigger += time.delta_seconds();
    if banshee.failed_timer > 0.0 {
        banshee.failed_timer = (banshee.failed_timer - time.delta_seconds()).max(0.0);
        if banshee.failed_timer == 0.0 {
            banshee.stage = 0;
            banshee.time_since_trigger = 0.0;
        }
        status.state = ExorcismState::Failed;
        return;
    }

    status.state = ExorcismState::Stage(banshee.stage);
    status.stage = banshee.stage;
    status.progress = if sequence_len == 0 {
        0.0
    } else {
        banshee.stage as f32 / sequence_len as f32
    };
    status.stacks = 0.0;
    status.max_stacks = 0.0;

    if !keys.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let mut closest: Option<(u8, f32, Vec3)> = None;
    for (anchor, transform) in anchors.iter() {
        let distance = player_transform
            .translation
            .distance(transform.translation);
        if distance <= tables.banshee.interact_distance {
            if closest.is_none() || distance < closest.unwrap().1 {
                closest = Some((anchor.index, distance, transform.translation));
            }
        }
    }

    let Some((index, _, _pos)) = closest else {
        return;
    };

    let expected = banshee.stage;
    let order_ok = index == expected;
    let timing_ok = if expected == 0 {
        true
    } else {
        let t = banshee.time_since_trigger;
        t >= tables.banshee.timing_min && t <= tables.banshee.timing_max
    };

    let next_state = rules::banshee_advance(banshee.stage, sequence_len, timing_ok, order_ok);
    match next_state {
        ExorcismState::Failed => {
            banshee.failed_timer = tables.banshee.fail_reset_seconds;
            banshee.stage = 0;
            status.state = ExorcismState::Failed;
        }
        ExorcismState::Complete => {
            if ghost_type.active == GhostType::Banshee {
                status.state = ExorcismState::Complete;
            } else {
                status.state = ExorcismState::Failed;
            }
            banshee.stage = sequence_len;
        }
        ExorcismState::Stage(next) => {
            banshee.stage = next;
            banshee.time_since_trigger = 0.0;
            status.state = ExorcismState::Stage(next);
        }
        _ => {}
    }
}

fn update_onryo_puzzle(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<InvestigationState>,
    tables: Res<ExorcismTables>,
    journal: Res<JournalState>,
    player: Query<
        &Transform,
        (
            With<crate::gameplay::investigator::Player>,
            Without<OnryoCursed>,
            Without<OnryoRitual>,
        ),
    >,
    mut cursed: Query<
        (Entity, &mut Transform, &mut OnryoCursed),
        (
            Without<crate::gameplay::investigator::Player>,
            Without<OnryoRitual>,
        ),
    >,
    rituals: Query<
        (&OnryoRitual, &Transform),
        (
            Without<crate::gameplay::investigator::Player>,
            Without<OnryoCursed>,
        ),
    >,
    mut onryo: ResMut<OnryoPuzzle>,
    mut status: ResMut<ExorcismStatus>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    let Some(puzzle_type) = investigation.guess else {
        return;
    };
    if !investigation.confirmed || puzzle_type != GhostType::Onryo {
        return;
    }

    let max_stacks = tables.onryo.max_stacks;
    let (new_stacks, failed) = rules::onryo_stack_tick(
        onryo.stacks,
        time.delta_seconds(),
        onryo.carrying.is_some(),
        max_stacks,
        tables.onryo.stack_rate,
    );
    onryo.stacks = new_stacks;
    onryo.max_stacks = max_stacks;
    if failed {
        status.state = ExorcismState::Failed;
        return;
    }

    status.state = ExorcismState::Stage(onryo.stage);
    status.stage = onryo.stage;
    status.stacks = onryo.stacks;
    status.max_stacks = onryo.max_stacks;

    if !keys.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    if let Some(carried_entity) = onryo.carrying {
        if let Ok((_, mut transform, cursed_obj)) = cursed.get_mut(carried_entity) {
            if !cursed_obj.placed {
                transform.translation =
                    player_transform.translation + Vec3::new(0.0, tables.onryo.carry_height, 0.0);
            }
        }
    }

    if onryo.carrying.is_none() {
        for (entity, transform, cursed_obj) in cursed.iter_mut() {
            if cursed_obj.placed {
                continue;
            }
            let distance = player_transform.translation.distance(transform.translation);
            if distance <= tables.onryo.interact_distance {
                onryo.carrying = Some(entity);
                onryo.stacks += tables.onryo.stack_penalty_wrong;
                break;
            }
        }
        return;
    }

    let mut target_spot: Option<(u8, Vec3)> = None;
    for (ritual, transform) in rituals.iter() {
        let distance = player_transform.translation.distance(transform.translation);
        if distance <= tables.onryo.interact_distance {
            target_spot = Some((ritual.index, transform.translation));
            break;
        }
    }

    let Some((spot_index, spot_pos)) = target_spot else {
        return;
    };

    let Some(carried_entity) = onryo.carrying.take() else {
        return;
    };

    if spot_index == onryo.stage {
        if let Ok((_, mut transform, mut cursed_obj)) = cursed.get_mut(carried_entity) {
            cursed_obj.placed = true;
            transform.translation = spot_pos + Vec3::new(0.0, 0.35, 0.0);
        }
        onryo.stage += 1;
        onryo.stacks = (onryo.stacks - tables.onryo.stack_reward_correct).max(0.0);
        let sequence_len = tables.onryo.ritual_positions.len() as u8;
        if onryo.stage >= sequence_len {
            if ghost_type.active == GhostType::Onryo {
                status.state = ExorcismState::Complete;
            } else {
                status.state = ExorcismState::Failed;
            }
        }
    } else {
        onryo.stacks += tables.onryo.stack_penalty_wrong;
    }
}
