use crate::prelude::*;

use crate::core::{GameRng, GhostType, InputMap, MenuState, Role, RoleState};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::map::rooms::{default_house_layout, room_id, room_id_in_house};
use crate::gameplay::map::{ExorcismPlacement, HouseLayout};

pub mod banshee;
pub mod onryo;
pub mod rules;
pub mod spirit;
pub mod tables;

use banshee::{BansheeAnchor, BansheePuzzle};
use onryo::{OnryoCursed, OnryoPuzzle, OnryoRitual};
use spirit::{SpiritAnchor, SpiritPuzzle};
use tables::ExorcismTables;

pub use banshee::BansheeSequence;
#[cfg(test)]
pub use banshee::BansheeNodeColor;
pub use spirit::SpiritMarker;

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

#[derive(Resource, Default)]
pub struct RoomLights {
    rooms: Vec<(u8, bool)>,
}

impl RoomLights {
    pub fn is_enabled(&self, room_id: u8) -> bool {
        self.rooms
            .iter()
            .find(|(id, _)| *id == room_id)
            .map(|(_, enabled)| *enabled)
            .unwrap_or(true)
    }

    pub fn reset(&mut self, house_layout: Option<&HouseLayout>) {
        self.rooms.clear();
        if let Some(layout) = house_layout {
            for room in &layout.rooms {
                self.rooms.push((room.id, true));
            }
            return;
        }
        for id in 0..4 {
            self.rooms.push((id, true));
        }
    }
}

#[derive(Component)]
pub(crate) struct PuzzleEntity;

pub struct ExorcismPlugin;

impl Plugin for ExorcismPlugin {
    fn build(&self, app: &mut App) {
        let tables = ExorcismTables::default();
        app.init_resource::<GameRng>()
            .init_resource::<crate::core::InputMap>()
            .insert_resource(tables.clone())
            .insert_resource(PuzzleSpawned(false))
            .insert_resource(RoomLights { rooms: Vec::new() })
            .insert_resource(SpiritPuzzle::default())
            .insert_resource(BansheeSequence::default())
            .insert_resource(BansheePuzzle::default())
            .insert_resource(OnryoPuzzle::new(tables.onryo.max_stacks))
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
                    spirit::update_spirit_puzzle,
                    banshee::update_banshee_puzzle,
                    onryo::update_onryo_puzzle,
                )
                    .chain(),
            );
    }
}

pub(crate) fn resolve_room_id(house_layout: Option<&HouseLayout>, position: Vec3) -> u8 {
    house_layout
        .and_then(|layout| room_id_in_house(layout, position))
        .unwrap_or_else(|| room_id(position))
}

fn toggle_room_light(lights: &mut RoomLights, room_id: u8) {
    if let Some((_, enabled)) = lights.rooms.iter_mut().find(|(id, _)| *id == room_id) {
        *enabled = !*enabled;
    } else {
        lights.rooms.push((room_id, false));
    }
}

pub(crate) fn spirit_anchor_positions(
    placement: Option<&ExorcismPlacement>,
    layout: Option<&HouseLayout>,
) -> Vec<Vec3> {
    placement
        .map(|p| p.spirit_anchors.clone())
        .or_else(|| layout.map(|l| l.exorcism.spirit_anchors.clone()))
        .unwrap_or_else(|| default_house_layout().exorcism.spirit_anchors)
}

pub(crate) fn banshee_anchor_positions(
    placement: Option<&ExorcismPlacement>,
    layout: Option<&HouseLayout>,
) -> Vec<Vec3> {
    placement
        .map(|p| p.banshee_anchors.clone())
        .or_else(|| layout.map(|l| l.exorcism.banshee_anchors.clone()))
        .unwrap_or_else(|| default_house_layout().exorcism.banshee_anchors)
}

pub(crate) fn onryo_cursed_positions(
    placement: Option<&ExorcismPlacement>,
    layout: Option<&HouseLayout>,
) -> Vec<Vec3> {
    placement
        .map(|p| p.onryo_cursed_positions.clone())
        .or_else(|| layout.map(|l| l.exorcism.onryo_cursed_positions.clone()))
        .unwrap_or_else(|| default_house_layout().exorcism.onryo_cursed_positions)
}

pub(crate) fn onryo_ritual_positions(
    placement: Option<&ExorcismPlacement>,
    layout: Option<&HouseLayout>,
) -> Vec<Vec3> {
    placement
        .map(|p| p.onryo_ritual_positions.clone())
        .or_else(|| layout.map(|l| l.exorcism.onryo_ritual_positions.clone()))
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
    placement: Option<Res<ExorcismPlacement>>,
    mut rng: ResMut<GameRng>,
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
        .reset_for_anchor_count(banshee_anchor_positions(placement.as_deref(), house_layout.as_deref()).len(), &mut rng);
    onryo.stage = 0;
    onryo.stacks = 0.0;
    onryo.carrying = None;
    lights.reset(house_layout.as_deref());

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
    placement: Option<Res<ExorcismPlacement>>,
) {
    if spawned.0 || menu.open || !investigation.confirmed {
        return;
    }

    let Some(puzzle_type) = investigation.guess else {
        return;
    };

    match puzzle_type {
        GhostType::Spirit => {
            let anchor_mesh = meshes.add(Cuboid::new(0.35, 1.4, 0.1));
            let anchor_material = materials.add(Color::srgb(0.7, 0.75, 0.9));
            let marker_mesh = meshes.add(Cylinder::new(0.6, 0.04));
            let marker_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.25, 0.55, 0.95),
                emissive: Color::srgb(0.12, 0.22, 0.45).into(),
                ..default()
            });
            for pos in spirit_anchor_positions(placement.as_deref(), house_layout.as_deref()) {
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
        GhostType::Banshee => {
            let anchor_mesh = meshes.add(Sphere::new(0.3).mesh().uv(16, 12));
            let marker_mesh = meshes.add(Cylinder::new(0.72, 0.05));
            for (index, pos) in banshee_anchor_positions(placement.as_deref(), house_layout.as_deref())
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
        GhostType::Onryo => {
            let cursed_mesh = meshes.add(Sphere::new(0.25).mesh().uv(16, 12));
            let cursed_material = materials.add(Color::srgb(0.9, 0.35, 0.35));
            let ritual_mesh = meshes.add(Cuboid::new(0.9, 0.05, 0.9));
            let ritual_material = materials.add(Color::srgb(0.35, 0.6, 0.9));

            for pos in onryo_cursed_positions(placement.as_deref(), house_layout.as_deref()) {
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

            for (index, pos) in onryo_ritual_positions(placement.as_deref(), house_layout.as_deref())
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
    input: Res<InputMap>,
    ghost: Res<GhostState>,
    mut lights: ResMut<RoomLights>,
    house_layout: Option<Res<HouseLayout>>,
) {
    if menu.open || role.current != Role::Ghost {
        return;
    }
    if keys.just_pressed(input.toggle_lights) {
        let room = resolve_room_id(house_layout.as_deref(), ghost.position);
        toggle_room_light(&mut lights, room);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::InputMap;

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
        app.insert_resource(crate::core::JournalState { open: false });
        app.insert_resource(crate::core::GhostTypeState {
            selected: GhostType::Banshee,
            active: GhostType::Banshee,
        });
        app.insert_resource(GhostState {
            position: Vec3::ZERO,
        });
        app.insert_resource(InputMap::default());
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
