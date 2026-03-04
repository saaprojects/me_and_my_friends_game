use crate::prelude::*;

use crate::gameplay::exorcism::RoomLights;
use crate::gameplay::ghost::GhostMarker;
use crate::gameplay::investigator::Player;
use std::path::Path;

use super::components::{Bounds, HouseLayout, Obstacle};

#[derive(Component)]
pub struct LayoutWall;

#[derive(Component)]
pub struct LayoutRoof;

#[derive(Component)]
pub struct EnvironmentShell;

#[derive(Component)]
pub struct EnvironmentDecor;

#[derive(Component)]
pub struct EnvironmentSetDressing;

#[derive(Component)]
pub struct LayoutProp;

#[derive(Component)]
pub struct RoomLightVisual {
    room_id: u8,
    on_intensity: f32,
    off_intensity: f32,
    last_enabled: bool,
    flicker_active: bool,
    flicker_elapsed: f32,
    flicker_duration: f32,
    flicker_from: f32,
    flicker_to: f32,
    flicker_phase: f32,
}

pub fn default_house_layout() -> HouseLayout {
    HouseLayout::two_room()
}

const TWO_ROOM_SHELL_SCENE: &str = "environment/house_shell_two_room.glb#Scene0";
const THREE_ROOM_SHELL_SCENE: &str = "environment/house_shell_three_room.glb#Scene0";
const SHARED_SHELL_SCENE: &str = "environment/house_shell.glb#Scene0";
const HOUSE_DECOR_SCENE: &str = "environment/house_decor.glb#Scene0";
const WALL_SCENE: &str = "house_assets/wall.glb#Scene0";
const ROOF_THICKNESS: f32 = 0.35;
const ROOF_OVERHANG_PER_SIDE: f32 = 0.12;
const ROOF_WALL_OVERLAP_Y: f32 = 0.08;
const WALL_HORIZONTAL_OVERLAP: f32 = 0.06;
const WALL_VERTICAL_OVERLAP: f32 = 0.15;
const ENABLE_WALL_SCENE_SKIN: bool = false;
// `house_assets/wall.glb` native bounds:
// X: [-0.05, 0.05], Y: [0.0, 2.4], Z: [-1.0, 1.0].
const WALL_ASSET_BASE_SIZE: Vec3 = Vec3::new(0.10000004, 2.4, 2.0);

fn scene_asset_file(scene_path: &str) -> &str {
    scene_path.split('#').next().unwrap_or(scene_path)
}

fn scene_exists(scene_path: &str) -> bool {
    let file = scene_asset_file(scene_path);
    Path::new("assets").join(file).exists() || Path::new("client").join("assets").join(file).exists()
}

fn choose_shell_scene(
    room_count: usize,
    has_two_room_scene: bool,
    has_three_room_scene: bool,
    has_shared_scene: bool,
) -> &'static str {
    if room_count == 3 && has_three_room_scene {
        return THREE_ROOM_SHELL_SCENE;
    }
    if room_count != 3 && has_two_room_scene {
        return TWO_ROOM_SHELL_SCENE;
    }
    if has_shared_scene {
        return SHARED_SHELL_SCENE;
    }
    if room_count == 3 {
        THREE_ROOM_SHELL_SCENE
    } else {
        TWO_ROOM_SHELL_SCENE
    }
}

fn shell_scene_for_house(house: &HouseLayout) -> &'static str {
    choose_shell_scene(
        house.rooms.len(),
        scene_exists(TWO_ROOM_SHELL_SCENE),
        scene_exists(THREE_ROOM_SHELL_SCENE),
        scene_exists(SHARED_SHELL_SCENE),
    )
}

fn spawn_environment_shell(
    commands: &mut Commands,
    asset_server: &AssetServer,
    house: &HouseLayout,
) -> bool {
    let shell_scene = shell_scene_for_house(house);
    if !scene_exists(shell_scene) {
        return false;
    }

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(shell_scene),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        EnvironmentShell,
    ));
    true
}

fn spawn_environment_decor(commands: &mut Commands, asset_server: &AssetServer) {
    if !scene_exists(HOUSE_DECOR_SCENE) {
        return;
    }

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(HOUSE_DECOR_SCENE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        EnvironmentDecor,
    ));
}

fn spawn_scene_if_exists(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene_path: &str,
    transform: Transform,
) -> bool {
    if !scene_exists(scene_path) {
        return false;
    }

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(scene_path.to_string()),
            transform,
            ..default()
        },
        EnvironmentSetDressing,
    ));
    true
}

fn spawn_fallback_props(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let prop_mesh_a = meshes.add(Cuboid::new(2.2, 1.2, 1.0));
    let prop_mesh_b = meshes.add(Cuboid::new(1.4, 0.8, 1.4));
    let prop_material_a = materials.add(Color::srgb(0.12, 0.16, 0.22));
    let prop_material_b = materials.add(Color::srgb(0.08, 0.1, 0.15));
    commands.spawn((
        PbrBundle {
            mesh: prop_mesh_a,
            material: prop_material_a,
            transform: Transform::from_xyz(-3.5, 0.6, -1.0),
            ..default()
        },
        LayoutProp,
    ));
    commands.spawn((
        PbrBundle {
            mesh: prop_mesh_b,
            material: prop_material_b,
            transform: Transform::from_xyz(4.0, 0.4, 3.0),
            ..default()
        },
        LayoutProp,
    ));
}

fn spawn_curated_set_dressing(
    commands: &mut Commands,
    asset_server: &AssetServer,
    house: &HouseLayout,
) -> bool {
    const INTERIOR: &str = "interior_assets/Models/GLTF format";

    let mut spawned_any = false;
    let mut spawn = |scene: &str, translation: Vec3, yaw: f32, scale: Vec3| {
        let path = format!("{scene}#Scene0");
        spawned_any |= spawn_scene_if_exists(
            commands,
            asset_server,
            &path,
            Transform {
                translation,
                rotation: Quat::from_rotation_y(yaw),
                scale,
            },
        );
    };

    // Living room cluster (main room)
    spawn(
        &format!("{INTERIOR}/loungeSofa.glb"),
        Vec3::new(-6.2, 0.0, -5.8),
        std::f32::consts::FRAC_PI_2,
        Vec3::splat(1.35),
    );
    spawn(
        &format!("{INTERIOR}/tableCoffee.glb"),
        Vec3::new(-5.2, 0.0, -5.7),
        0.0,
        Vec3::splat(1.35),
    );
    spawn(
        &format!("{INTERIOR}/rugRectangle.glb"),
        Vec3::new(-5.5, 0.01, -5.7),
        0.0,
        Vec3::splat(1.45),
    );
    spawn(
        &format!("{INTERIOR}/bookcaseClosedWide.glb"),
        Vec3::new(-8.3, 0.0, -3.2),
        std::f32::consts::FRAC_PI_2,
        Vec3::splat(1.3),
    );

    // Side room cluster
    spawn(
        &format!("{INTERIOR}/kitchenCabinet.glb"),
        Vec3::new(7.0, 0.0, -7.0),
        std::f32::consts::PI,
        Vec3::splat(1.25),
    );
    spawn(
        &format!("{INTERIOR}/kitchenFridge.glb"),
        Vec3::new(8.2, 0.0, -7.2),
        std::f32::consts::PI,
        Vec3::splat(1.25),
    );
    spawn(
        &format!("{INTERIOR}/cabinetTelevision.glb"),
        Vec3::new(4.0, 0.0, 3.0),
        std::f32::consts::PI,
        Vec3::splat(1.35),
    );
    spawn(
        &format!("{INTERIOR}/plantSmall2.glb"),
        Vec3::new(6.8, 0.0, 5.8),
        0.0,
        Vec3::splat(1.4),
    );
    spawn(
        &format!("{INTERIOR}/lampSquareFloor.glb"),
        Vec3::new(5.6, 0.0, 4.8),
        0.0,
        Vec3::splat(1.4),
    );

    // Upper room dressing only for 3-room layout
    if house.rooms.len() >= 3 {
        spawn(
            &format!("{INTERIOR}/desk.glb"),
            Vec3::new(-4.2, 0.0, 6.6),
            std::f32::consts::PI,
            Vec3::splat(1.35),
        );
        spawn(
            &format!("{INTERIOR}/chairDesk.glb"),
            Vec3::new(-4.2, 0.0, 5.8),
            0.0,
            Vec3::splat(1.35),
        );
        spawn(
            &format!("{INTERIOR}/lampSquareCeiling.glb"),
            Vec3::new(-4.0, 3.1, 6.0),
            0.0,
            Vec3::splat(1.5),
        );
    }

    spawned_any
}

pub fn room_id_in_house(layout: &HouseLayout, position: Vec3) -> Option<u8> {
    layout
        .rooms
        .iter()
        .find(|room| room.bounds.contains_xz(position))
        .map(|room| room.id)
}

pub fn investigator_spawn_position() -> Vec3 {
    default_house_layout().investigator_spawn
}

#[allow(dead_code)]
pub fn ghost_spawn_positions() -> Vec<Vec3> {
    default_house_layout().ghost_spawns
}

pub fn random_ghost_spawn_position() -> Vec3 {
    default_house_layout().random_ghost_spawn()
}

pub(crate) fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    house: Res<HouseLayout>,
    room_lights: Option<Res<RoomLights>>,
) {
    let floor_mesh = meshes.add(Cuboid::new(20.0, 0.2, 20.0));
    let floor_material = materials.add(Color::srgb(0.06, 0.1, 0.16));
    commands.spawn(PbrBundle {
        mesh: floor_mesh,
        material: floor_material,
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    let shell_loaded = spawn_environment_shell(&mut commands, &asset_server, &house);
    if !shell_loaded {
        spawn_layout_walls(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            &house,
        );
        spawn_layout_roof(&mut commands, &mut meshes, &mut materials, &house);
    }
    spawn_environment_decor(&mut commands, &asset_server);
    if !spawn_curated_set_dressing(&mut commands, &asset_server, &house) {
        spawn_fallback_props(&mut commands, &mut meshes, &mut materials);
    }
    spawn_room_lights(
        &mut commands,
        &house,
        room_lights.as_deref(),
    );

    let player_mesh = meshes.add(Cuboid::new(0.7, 1.8, 0.7));
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.65, 0.75),
        metallic: 0.0,
        perceptual_roughness: 0.95,
        reflectance: 0.02,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: player_mesh,
            material: player_material,
            transform: Transform::from_translation(house.investigator_spawn),
            ..default()
        },
        Player,
    ));

    let ghost_mesh = meshes.add(Sphere::new(0.18).mesh().uv(16, 12));
    let ghost_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.8, 1.0),
        emissive: Color::srgb(0.25, 0.35, 0.6).into(),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: ghost_mesh,
            material: ghost_material,
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..default()
        },
        GhostMarker,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.55, 0.62, 0.8),
            illuminance: 40.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(6.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.6, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_layout_walls(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    house: &HouseLayout,
) {
    let use_wall_scene = ENABLE_WALL_SCENE_SKIN && scene_exists(WALL_SCENE);
    for wall in &house.walls {
        let render_size = wall_render_size(wall.size);
        let render_center = wall_render_center(wall.translation, wall.size, render_size);
        // Seam-proof core wall: always present so floor/ceiling joins are airtight.
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(render_size.x, render_size.y, render_size.z)),
                material: materials.add(Color::srgb(wall.color[0], wall.color[1], wall.color[2])),
                transform: Transform::from_translation(render_center),
                ..default()
            },
            LayoutWall,
        ));

        // Decorative skin from house asset kit (optional).
        if use_wall_scene {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(WALL_SCENE),
                    transform: wall_scene_transform(render_center, render_size),
                    ..default()
                },
                LayoutWall,
            ));
        }
    }
}

fn wall_render_size(layout_size: Vec3) -> Vec3 {
    Vec3::new(
        layout_size.x + WALL_HORIZONTAL_OVERLAP * 2.0,
        (layout_size.y + WALL_VERTICAL_OVERLAP * 2.0).max(layout_size.y),
        layout_size.z + WALL_HORIZONTAL_OVERLAP * 2.0,
    )
}

fn wall_render_center(layout_center: Vec3, layout_size: Vec3, render_size: Vec3) -> Vec3 {
    Vec3::new(
        layout_center.x,
        layout_center.y + (render_size.y - layout_size.y) * 0.5,
        layout_center.z,
    )
}

fn wall_scene_transform(center: Vec3, render_size: Vec3) -> Transform {
    Transform {
        // Mesh origin is at the base (Y=0), not center, so offset down by half target height.
        translation: Vec3::new(center.x, center.y - render_size.y * 0.5, center.z),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(
            render_size.x / WALL_ASSET_BASE_SIZE.x,
            render_size.y / WALL_ASSET_BASE_SIZE.y,
            render_size.z / WALL_ASSET_BASE_SIZE.z,
        ),
    }
}

fn spawn_layout_roof(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    house: &HouseLayout,
) {
    let (roof_size_x, roof_size_z, roof_y) = roof_dimensions(house);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(roof_size_x, 0.35, roof_size_z)),
            material: materials.add(Color::srgb(0.12, 0.14, 0.18)),
            transform: Transform::from_xyz(0.0, roof_y, 0.0),
            ..default()
        },
        LayoutRoof,
    ));
}

fn roof_dimensions(house: &HouseLayout) -> (f32, f32, f32) {
    if house.walls.is_empty() {
        let roof_size_x = (house.bounds.max_x - house.bounds.min_x) + 0.4 + ROOF_OVERHANG_PER_SIDE * 2.0;
        let roof_size_z = (house.bounds.max_z - house.bounds.min_z) + 0.4 + ROOF_OVERHANG_PER_SIDE * 2.0;
        return (
            roof_size_x,
            roof_size_z,
            4.0 - ROOF_WALL_OVERLAP_Y + ROOF_THICKNESS * 0.5,
        );
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    let mut max_wall_top = f32::NEG_INFINITY;

    for wall in &house.walls {
        let half_x = wall.size.x * 0.5;
        let half_z = wall.size.z * 0.5;
        min_x = min_x.min(wall.translation.x - half_x);
        max_x = max_x.max(wall.translation.x + half_x);
        min_z = min_z.min(wall.translation.z - half_z);
        max_z = max_z.max(wall.translation.z + half_z);
        max_wall_top = max_wall_top.max(wall.translation.y + wall.size.y * 0.5);
    }

    (
        (max_x - min_x).max(0.1) + ROOF_OVERHANG_PER_SIDE * 2.0,
        (max_z - min_z).max(0.1) + ROOF_OVERHANG_PER_SIDE * 2.0,
        max_wall_top - ROOF_WALL_OVERLAP_Y + ROOF_THICKNESS * 0.5,
    )
}

fn room_light_range(bounds: Bounds) -> f32 {
    let width = bounds.max_x - bounds.min_x;
    let depth = bounds.max_z - bounds.min_z;
    (width.max(depth) * 0.9 + 4.0).clamp(10.0, 16.0)
}

fn spawn_room_lights(commands: &mut Commands, house: &HouseLayout, lights: Option<&RoomLights>) {
    const ON_INTENSITY: f32 = 250_000.0;
    const OFF_INTENSITY: f32 = 0.0;

    for room in &house.rooms {
        let width = room.bounds.max_x - room.bounds.min_x;
        let depth = room.bounds.max_z - room.bounds.min_z;
        let center_x = (room.bounds.min_x + room.bounds.max_x) * 0.5;
        let center_z = (room.bounds.min_z + room.bounds.max_z) * 0.5;
        let enabled = lights.map(|state| state.is_enabled(room.id)).unwrap_or(true);
        let initial_intensity = if enabled { ON_INTENSITY } else { OFF_INTENSITY };
        let mut fixture_positions = vec![Vec3::new(center_x, 3.3, center_z)];
        if depth > width * 1.35 {
            let z_a = room.bounds.min_z + depth * 0.33;
            let z_b = room.bounds.min_z + depth * 0.67;
            fixture_positions = vec![Vec3::new(center_x, 3.3, z_a), Vec3::new(center_x, 3.3, z_b)];
        } else if width > depth * 1.35 {
            let x_a = room.bounds.min_x + width * 0.33;
            let x_b = room.bounds.min_x + width * 0.67;
            fixture_positions = vec![Vec3::new(x_a, 3.3, center_z), Vec3::new(x_b, 3.3, center_z)];
        }

        for position in fixture_positions {
            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        color: Color::srgb(1.0, 0.98, 0.92),
                        intensity: initial_intensity,
                        range: room_light_range(room.bounds),
                        shadows_enabled: true,
                        shadow_depth_bias: 0.005,
                        shadow_normal_bias: 0.005,
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..default()
                },
                RoomLightVisual {
                    room_id: room.id,
                    on_intensity: ON_INTENSITY,
                    off_intensity: OFF_INTENSITY,
                    last_enabled: enabled,
                    flicker_active: false,
                    flicker_elapsed: 0.0,
                    flicker_duration: 0.42
                        + ((room.id as f32 * 1.37 + position.x * 0.17 + position.z * 0.11).sin()
                            .abs()
                            * 0.16),
                    flicker_from: initial_intensity,
                    flicker_to: initial_intensity,
                    flicker_phase: room.id as f32 * 1.37 + position.x * 0.17 + position.z * 0.11,
                },
            ));
        }
    }
}

pub(crate) fn sync_layout_walls(
    house: Res<HouseLayout>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    room_lights: Option<Res<RoomLights>>,
    walls: Query<Entity, With<LayoutWall>>,
    roofs: Query<Entity, With<LayoutRoof>>,
    shell_entities: Query<Entity, With<EnvironmentShell>>,
    decor_entities: Query<Entity, With<EnvironmentDecor>>,
    set_dressing_entities: Query<Entity, With<EnvironmentSetDressing>>,
    layout_props: Query<Entity, With<LayoutProp>>,
    room_lights_visuals: Query<Entity, With<RoomLightVisual>>,
) {
    if !house.is_changed() {
        return;
    }

    for entity in walls.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in roofs.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in shell_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in decor_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in set_dressing_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in layout_props.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in room_lights_visuals.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let shell_loaded = spawn_environment_shell(&mut commands, &asset_server, &house);
    if !shell_loaded {
        spawn_layout_walls(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            &house,
        );
        spawn_layout_roof(&mut commands, &mut meshes, &mut materials, &house);
    }
    spawn_environment_decor(&mut commands, &asset_server);
    if !spawn_curated_set_dressing(&mut commands, &asset_server, &house) {
        spawn_fallback_props(&mut commands, &mut meshes, &mut materials);
    }
    spawn_room_lights(
        &mut commands,
        &house,
        room_lights.as_deref(),
    );
}

pub(crate) fn sync_room_light_visuals(
    lights: Option<Res<RoomLights>>,
    mut room_lights: Query<(&mut RoomLightVisual, &mut PointLight)>,
) {
    let Some(lights) = lights else {
        return;
    };
    if !lights.is_changed() {
        return;
    }

    for (mut visual, mut point_light) in room_lights.iter_mut() {
        let enabled = lights.is_enabled(visual.room_id);
        let target = if enabled {
            visual.on_intensity
        } else {
            visual.off_intensity
        };
        if enabled != visual.last_enabled {
            visual.flicker_active = true;
            visual.flicker_elapsed = 0.0;
            visual.flicker_from = point_light.intensity;
            visual.flicker_to = target;
            visual.last_enabled = enabled;
            continue;
        }
        point_light.intensity = target;
        visual.flicker_from = target;
        visual.flicker_to = target;
    }
}

fn light_flicker_multiplier(elapsed: f32, phase: f32, turning_on: bool) -> f32 {
    let burst = (elapsed * (43.0 + phase * 0.25)).sin() * 0.5 + 0.5;
    let chatter = (elapsed * (76.0 + phase * 0.45)).cos() * 0.5 + 0.5;
    let mix = burst * 0.65 + chatter * 0.35;
    let gate = if turning_on {
        if (elapsed * (24.0 + phase * 0.15)).sin() > -0.08 {
            1.0
        } else {
            0.22
        }
    } else if (elapsed * (28.0 + phase * 0.2)).sin() > 0.25 {
        0.6
    } else {
        0.08
    };
    (mix * gate).clamp(0.02, 1.2)
}

pub(crate) fn animate_room_light_flicker(
    time: Res<Time>,
    lights: Option<Res<RoomLights>>,
    mut room_lights: Query<(&mut RoomLightVisual, &mut PointLight)>,
) {
    let Some(lights) = lights else {
        return;
    };

    let dt = time.delta_seconds();
    for (mut visual, mut point_light) in room_lights.iter_mut() {
        let enabled = lights.is_enabled(visual.room_id);
        let target = if enabled {
            visual.on_intensity
        } else {
            visual.off_intensity
        };

        if !visual.flicker_active {
            point_light.intensity = target;
            continue;
        }

        visual.flicker_elapsed += dt;
        let progress = (visual.flicker_elapsed / visual.flicker_duration).clamp(0.0, 1.0);
        let base = visual.flicker_from + (visual.flicker_to - visual.flicker_from) * progress;
        let turning_on = visual.flicker_to > visual.flicker_from;
        let flicker = light_flicker_multiplier(visual.flicker_elapsed, visual.flicker_phase, turning_on);
        let shaped = if turning_on {
            (0.22 + flicker * 0.95).min(1.25)
        } else {
            flicker
        };
        point_light.intensity = (base * shaped).max(0.0);

        if progress >= 1.0 {
            visual.flicker_active = false;
            visual.flicker_from = target;
            visual.flicker_to = target;
            point_light.intensity = target;
        }
    }
}

pub fn clamp_to_bounds(pos: &mut Vec3, bounds: Bounds, radius: f32) {
    pos.x = pos.x.clamp(bounds.min_x + radius, bounds.max_x - radius);
    pos.z = pos.z.clamp(bounds.min_z + radius, bounds.max_z - radius);
}

pub fn collides(pos: Vec3, radius: f32, obstacles: &[Obstacle]) -> bool {
    obstacles.iter().any(|obs| {
        let hit_x = pos.x + radius > obs.min_x && pos.x - radius < obs.max_x;
        let hit_z = pos.z + radius > obs.min_z && pos.z - radius < obs.max_z;
        hit_x && hit_z
    })
}

pub fn move_with_collisions(
    pos: &mut Vec3,
    movement: Vec3,
    radius: f32,
    bounds: Bounds,
    obstacles: &[Obstacle],
    block_interior: bool,
) {
    let mut next = *pos + movement;
    clamp_to_bounds(&mut next, bounds, radius);

    if !block_interior {
        *pos = next;
        return;
    }

    if !collides(next, radius, obstacles) {
        *pos = next;
        return;
    }

    let mut try_x = *pos;
    try_x.x = next.x;
    clamp_to_bounds(&mut try_x, bounds, radius);
    if !collides(try_x, radius, obstacles) {
        *pos = try_x;
        return;
    }

    let mut try_z = *pos;
    try_z.z = next.z;
    clamp_to_bounds(&mut try_z, bounds, radius);
    if !collides(try_z, radius, obstacles) {
        *pos = try_z;
    }
}

pub fn clamp_camera_distance(base: Vec3, dir: Vec3, desired: f32, bounds: Bounds) -> f32 {
    let margin = 0.6;
    let mut max_t = desired;

    if dir.x.abs() > f32::EPSILON {
        let bound_x = if dir.x > 0.0 {
            bounds.max_x - margin
        } else {
            bounds.min_x + margin
        };
        let t = (bound_x - base.x) / dir.x;
        if t.is_finite() && t > 0.0 {
            max_t = max_t.min(t);
        }
    }

    if dir.z.abs() > f32::EPSILON {
        let bound_z = if dir.z > 0.0 {
            bounds.max_z - margin
        } else {
            bounds.min_z + margin
        };
        let t = (bound_z - base.z) / dir.z;
        if t.is_finite() && t > 0.0 {
            max_t = max_t.min(t);
        }
    }

    max_t.clamp(1.2, desired)
}

pub fn avoid_camera_obstacles(
    base: Vec3,
    dir: Vec3,
    mut distance: f32,
    radius: f32,
    obstacles: &[Obstacle],
) -> f32 {
    let step = 0.2;
    while distance > 1.2 {
        let candidate = base + dir * distance;
        if !collides(candidate, radius, obstacles)
            && !segment_collides(base, candidate, radius, obstacles)
        {
            break;
        }
        distance -= step;
        if distance <= 1.2 {
            return 1.2;
        }
    }
    distance
}

fn segment_collides(start: Vec3, end: Vec3, radius: f32, obstacles: &[Obstacle]) -> bool {
    let delta = end - start;
    let len = delta.length();
    if len <= f32::EPSILON {
        return collides(start, radius, obstacles);
    }

    let dir = delta / len;
    let step = (radius * 0.5).max(0.05);
    let mut t = step;
    while t < len {
        let point = start + dir * t;
        if collides(point, radius, obstacles) {
            return true;
        }
        t += step;
    }
    false
}

#[cfg(test)]
#[path = "systems_tests.rs"]
mod systems_tests;

pub fn room_id(position: Vec3) -> u8 {
    let x = if position.x >= 0.0 { 1 } else { 0 };
    let z = if position.z >= 0.0 { 1 } else { 0 };
    (x << 1) | z
}

pub fn shortest_angle(current: f32, target: f32) -> f32 {
    let mut diff = target - current;
    while diff > std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }
    diff
}
