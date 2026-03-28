use crate::prelude::*;

use crate::core::{GhostType, GhostTypeState, JournalState, MenuState, Role, RoleState};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::map::systems::default_house_layout;
use crate::gameplay::map::systems::{room_id, room_id_in_house};
use crate::gameplay::map::HouseLayout;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod rules;
pub mod tables;

use tables::ExorcismTables;

static BANSHEE_SEQUENCE_COUNTER: AtomicU64 = AtomicU64::new(0);

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
pub(crate) struct RoomLights {
    rooms: Vec<(u8, bool)>,
}

impl RoomLights {
    pub(crate) fn is_enabled(&self, room_id: u8) -> bool {
        self.rooms
            .iter()
            .find(|(id, _)| *id == room_id)
            .map(|(_, enabled)| *enabled)
            .unwrap_or(true)
    }
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
    color: BansheeNodeColor,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BansheeNodeColor {
    Violet,
    Amber,
    Teal,
}

impl BansheeNodeColor {
    fn palette(count: usize) -> Vec<Self> {
        [Self::Violet, Self::Amber, Self::Teal]
            .into_iter()
            .take(count.clamp(1, 3))
            .collect()
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Violet => "Violet",
            Self::Amber => "Amber",
            Self::Teal => "Teal",
        }
    }

    fn base_color(self) -> Color {
        match self {
            Self::Violet => Color::srgb(0.8, 0.38, 0.95),
            Self::Amber => Color::srgb(0.95, 0.72, 0.25),
            Self::Teal => Color::srgb(0.25, 0.85, 0.8),
        }
    }

    fn emissive(self) -> Color {
        match self {
            Self::Violet => Color::srgb(0.25, 0.1, 0.38),
            Self::Amber => Color::srgb(0.34, 0.2, 0.06),
            Self::Teal => Color::srgb(0.06, 0.24, 0.22),
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct BansheeSequence {
    pub anchor_colors: Vec<BansheeNodeColor>,
    pub order: Vec<BansheeNodeColor>,
}

impl Default for BansheeSequence {
    fn default() -> Self {
        Self::for_anchor_count(3)
    }
}

impl BansheeSequence {
    pub fn for_anchor_count(count: usize) -> Self {
        let anchor_colors = BansheeNodeColor::palette(count);
        let mut order = anchor_colors.clone();
        shuffle_colors(&mut order, banshee_random_seed(count as u64));
        Self {
            anchor_colors,
            order,
        }
    }

    fn reset_for_anchor_count(&mut self, count: usize) {
        *self = Self::for_anchor_count(count);
    }

    pub fn sequence_len(&self) -> u8 {
        self.order.len() as u8
    }

    pub fn color_for_index(&self, index: usize) -> BansheeNodeColor {
        self.anchor_colors
            .get(index)
            .copied()
            .unwrap_or(BansheeNodeColor::Violet)
    }

    pub fn expected_color(&self, stage: u8) -> Option<BansheeNodeColor> {
        self.order.get(stage as usize).copied()
    }

    pub fn current_target_label(&self, stage: u8) -> &'static str {
        self.expected_color(stage)
            .map(BansheeNodeColor::label)
            .unwrap_or("Complete")
    }

    pub fn order_summary(&self) -> String {
        self.order
            .iter()
            .map(|color| color.label())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
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
            .insert_resource(RoomLights { rooms: Vec::new() })
            .insert_resource(SpiritPuzzle { progress: 0.0 })
            .insert_resource(BansheeSequence::default())
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
                )
                    .chain(),
            );
    }
}

fn resolve_room_id(house_layout: Option<&HouseLayout>, position: Vec3) -> u8 {
    house_layout
        .and_then(|layout| room_id_in_house(layout, position))
        .unwrap_or_else(|| room_id(position))
}

fn reset_room_lights(lights: &mut RoomLights, house_layout: Option<&HouseLayout>) {
    lights.rooms.clear();
    if let Some(layout) = house_layout {
        for room in &layout.rooms {
            lights.rooms.push((room.id, true));
        }
        return;
    }
    for id in 0..4 {
        lights.rooms.push((id, true));
    }
}

fn toggle_room_light(lights: &mut RoomLights, room_id: u8) {
    if let Some((_, enabled)) = lights.rooms.iter_mut().find(|(id, _)| *id == room_id) {
        *enabled = !*enabled;
    } else {
        lights.rooms.push((room_id, false));
    }
}

fn spirit_anchor_positions(house_layout: Option<&HouseLayout>) -> Vec<Vec3> {
    house_layout
        .map(|layout| layout.exorcism.spirit_anchors.clone())
        .unwrap_or_else(|| default_house_layout().exorcism.spirit_anchors)
}

fn banshee_anchor_positions(house_layout: Option<&HouseLayout>) -> Vec<Vec3> {
    house_layout
        .map(|layout| layout.exorcism.banshee_anchors.clone())
        .unwrap_or_else(|| default_house_layout().exorcism.banshee_anchors)
}

fn onryo_cursed_positions(house_layout: Option<&HouseLayout>) -> Vec<Vec3> {
    house_layout
        .map(|layout| layout.exorcism.onryo_cursed_positions.clone())
        .unwrap_or_else(|| default_house_layout().exorcism.onryo_cursed_positions)
}

fn onryo_ritual_positions(house_layout: Option<&HouseLayout>) -> Vec<Vec3> {
    house_layout
        .map(|layout| layout.exorcism.onryo_ritual_positions.clone())
        .unwrap_or_else(|| default_house_layout().exorcism.onryo_ritual_positions)
}

fn maybe_reset_puzzle(
    spawned: Res<PuzzleSpawned>,
    tables: Res<ExorcismTables>,
    mut status: ResMut<ExorcismStatus>,
    mut spirit: ResMut<SpiritPuzzle>,
    mut banshee: ResMut<BansheePuzzle>,
    mut banshee_sequence: ResMut<BansheeSequence>,
    mut onryo: ResMut<OnryoPuzzle>,
    mut lights: ResMut<RoomLights>,
    mut anchors: Query<Entity, With<PuzzleEntity>>,
    mut commands: Commands,
    menu: Res<MenuState>,
    investigation: Res<InvestigationState>,
    house_layout: Option<Res<HouseLayout>>,
) {
    if menu.open {
        return;
    }
    // Only reset state when PuzzleSpawned was explicitly flipped to false
    // (new haunt/journal confirm). Otherwise this system would overwrite
    // ghost-controlled room lights every frame.
    if spawned.0 || !spawned.is_changed() {
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
    banshee_sequence
        .reset_for_anchor_count(banshee_anchor_positions(house_layout.as_deref()).len());
    onryo.stage = 0;
    onryo.stacks = 0.0;
    onryo.carrying = None;
    reset_room_lights(&mut lights, house_layout.as_deref());

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
    banshee_sequence: Res<BansheeSequence>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    house_layout: Option<Res<HouseLayout>>,
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
            for pos in spirit_anchor_positions(house_layout.as_deref()) {
                commands.spawn((
                    PbrBundle {
                        mesh: anchor_mesh.clone(),
                        material: anchor_material.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    SpiritAnchor {
                        room_id: resolve_room_id(house_layout.as_deref(), pos),
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
            let marker_mesh = meshes.add(Cylinder::new(0.72, 0.05));
            for (index, pos) in banshee_anchor_positions(house_layout.as_deref())
                .into_iter()
                .enumerate()
            {
                let color = banshee_sequence.color_for_index(index);
                let anchor_material = materials.add(StandardMaterial {
                    base_color: color.base_color(),
                    emissive: color.emissive().into(),
                    perceptual_roughness: 0.42,
                    ..default()
                });
                let marker_material = materials.add(StandardMaterial {
                    base_color: color.base_color(),
                    emissive: color.emissive().into(),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                });
                commands.spawn((
                    PbrBundle {
                        mesh: anchor_mesh.clone(),
                        material: anchor_material,
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    BansheeAnchor { color },
                    PuzzleEntity,
                ));
                commands.spawn((
                    PbrBundle {
                        mesh: marker_mesh.clone(),
                        material: marker_material,
                        transform: Transform::from_translation(Vec3::new(pos.x, 0.03, pos.z)),
                        ..default()
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

            for pos in onryo_cursed_positions(house_layout.as_deref()) {
                commands.spawn((
                    PbrBundle {
                        mesh: cursed_mesh.clone(),
                        material: cursed_material.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    OnryoCursed { placed: false },
                    PuzzleEntity,
                ));
            }

            for (index, pos) in onryo_ritual_positions(house_layout.as_deref())
                .into_iter()
                .enumerate()
            {
                commands.spawn((
                    PbrBundle {
                        mesh: ritual_mesh.clone(),
                        material: ritual_material.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    OnryoRitual { index: index as u8 },
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
    house_layout: Option<Res<HouseLayout>>,
) {
    if menu.open || role.current != Role::Ghost {
        return;
    }
    if keys.just_pressed(KeyCode::KeyL) {
        let room = resolve_room_id(house_layout.as_deref(), ghost.position);
        toggle_room_light(&mut lights, room);
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

fn update_banshee_puzzle(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<InvestigationState>,
    tables: Res<ExorcismTables>,
    banshee_sequence: Res<BansheeSequence>,
    journal: Res<JournalState>,
    house_layout: Option<Res<HouseLayout>>,
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

    let layout_sequence_len = house_layout
        .as_ref()
        .map(|layout| layout.exorcism.banshee_anchors.len() as u8)
        .unwrap_or_else(|| tables.banshee.sequence_len());
    let sequence_len = banshee_sequence
        .sequence_len()
        .max(layout_sequence_len)
        .max(1);
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
    status.max_stacks = sequence_len as f32;

    if !keys.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let mut closest: Option<(BansheeNodeColor, f32, Vec3)> = None;
    for (anchor, transform) in anchors.iter() {
        let distance = player_transform.translation.distance(transform.translation);
        if distance <= tables.banshee.interact_distance {
            if closest.is_none() || distance < closest.unwrap().1 {
                closest = Some((anchor.color, distance, transform.translation));
            }
        }
    }

    let Some((color, _, _pos)) = closest else {
        return;
    };

    let expected = banshee.stage;
    let order_ok = banshee_sequence.expected_color(expected) == Some(color);
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
    house_layout: Option<Res<HouseLayout>>,
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
        let sequence_len = house_layout
            .as_ref()
            .map(|layout| layout.exorcism.onryo_ritual_positions.len() as u8)
            .unwrap_or(tables.onryo.default_ritual_count);
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

fn banshee_random_seed(salt: u64) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0);
    let counter = BANSHEE_SEQUENCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut seed = nanos ^ counter.rotate_left(13) ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    if seed == 0 {
        seed = 0x94D0_49BB_1331_11EB;
    }
    seed
}

fn shuffle_colors(colors: &mut [BansheeNodeColor], mut seed: u64) {
    for index in (1..colors.len()).rev() {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap_index = (seed % (index as u64 + 1)) as usize;
        colors.swap(index, swap_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banshee_accepts_second_correct_press_after_valid_delay() {
        let mut app = App::new();
        app.add_plugins(ExorcismPlugin);
        app.insert_resource(MenuState {
            open: false,
            selected_role: Role::Investigator,
        });
        app.insert_resource(RoleState {
            current: Role::Investigator,
        });
        app.insert_resource(JournalState { open: false });
        app.insert_resource(GhostTypeState {
            selected: GhostType::Banshee,
            active: GhostType::Banshee,
        });
        app.insert_resource(GhostState {
            position: Vec3::ZERO,
        });
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(Time::<()>::default());
        app.insert_resource(Assets::<Mesh>::default());
        app.insert_resource(Assets::<StandardMaterial>::default());
        app.insert_resource(HouseLayout {
            bounds: crate::gameplay::map::components::Bounds {
                min_x: -10.0,
                max_x: 10.0,
                min_z: -10.0,
                max_z: 10.0,
            },
            obstacles: Vec::new(),
            rooms: vec![crate::gameplay::map::components::RoomZone {
                id: 0,
                name: "Only Room",
                bounds: crate::gameplay::map::components::Bounds {
                    min_x: -10.0,
                    max_x: 10.0,
                    min_z: -10.0,
                    max_z: 10.0,
                },
            }],
            walls: Vec::new(),
            exorcism: crate::gameplay::map::components::ExorcismLayout {
                spirit_anchors: vec![Vec3::new(0.0, 0.7, 0.0)],
                banshee_anchors: vec![
                    Vec3::new(-4.0, 0.5, 0.0),
                    Vec3::new(0.0, 0.5, 0.0),
                    Vec3::new(4.0, 0.5, 0.0),
                ],
                onryo_cursed_positions: vec![Vec3::new(0.0, 0.4, 0.0)],
                onryo_ritual_positions: vec![Vec3::new(0.0, 0.1, 0.0)],
            },
            investigator_spawn: Vec3::new(-4.0, 0.9, 0.0),
            investigator_spawns: Vec::new(),
            ghost_spawns: vec![Vec3::new(6.0, 1.6, 0.0)],
        });

        app.world_mut().spawn((
            Transform::from_xyz(-4.0, 0.9, 0.0),
            GlobalTransform::default(),
            crate::gameplay::investigator::Player,
        ));

        {
            let mut investigation = app.world_mut().resource_mut::<InvestigationState>();
            investigation.guess = Some(GhostType::Banshee);
            investigation.confirmed = true;
        }

        app.update();

        let sequence = app.world().resource::<BansheeSequence>().clone();
        let mut anchors = app
            .world_mut()
            .query::<(&BansheeAnchor, &Transform)>()
            .iter(app.world())
            .map(|(anchor, transform)| (anchor.color, transform.translation))
            .collect::<Vec<_>>();
        anchors.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
        let first_target = anchors
            .iter()
            .find(|(color, _)| *color == sequence.order[0])
            .map(|(_, position)| *position)
            .unwrap();
        let second_target = anchors
            .iter()
            .find(|(color, _)| *color == sequence.order[1])
            .map(|(_, position)| *position)
            .unwrap();

        {
            let mut player = app
                .world_mut()
                .query_filtered::<&mut Transform, With<crate::gameplay::investigator::Player>>();
            let mut transform = player.single_mut(app.world_mut());
            transform.translation = Vec3::new(first_target.x, 0.9, first_target.z);
        }
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.press(KeyCode::KeyF);
        }
        app.update();
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.release(KeyCode::KeyF);
        }
        app.insert_resource(ButtonInput::<KeyCode>::default());

        assert_eq!(app.world().resource::<BansheePuzzle>().stage, 1);

        {
            let mut player = app
                .world_mut()
                .query_filtered::<&mut Transform, With<crate::gameplay::investigator::Player>>();
            let mut transform = player.single_mut(app.world_mut());
            transform.translation = Vec3::new(second_target.x, 0.9, second_target.z);
        }
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(std::time::Duration::from_secs_f32(1.0));
        }
        app.update();
        app.insert_resource(ButtonInput::<KeyCode>::default());
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.press(KeyCode::KeyF);
        }
        app.update();

        let banshee = app.world().resource::<BansheePuzzle>();
        assert_eq!(
            banshee.stage, 2,
            "time_since_trigger={}",
            banshee.time_since_trigger
        );
        assert_eq!(
            app.world().resource::<ExorcismStatus>().state,
            ExorcismState::Stage(2)
        );
    }
}
