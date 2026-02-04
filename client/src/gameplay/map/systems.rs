use crate::prelude::*;

use crate::gameplay::ghost::GhostMarker;
use crate::gameplay::investigator::Player;

use super::components::{Bounds, Obstacle};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mesh = meshes.add(Cuboid::new(20.0, 0.2, 20.0));
    let floor_material = materials.add(Color::srgb(0.06, 0.1, 0.16));
    commands.spawn(PbrBundle {
        mesh: floor_mesh,
        material: floor_material,
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    let wall_material_a = materials.add(Color::srgb(0.08, 0.1, 0.15));
    let wall_material_b = materials.add(Color::srgb(0.06, 0.08, 0.13));
    let wall_mesh_x = meshes.add(Cuboid::new(20.0, 4.0, 0.4));
    let wall_mesh_z = meshes.add(Cuboid::new(0.4, 4.0, 20.0));
    commands.spawn(PbrBundle {
        mesh: wall_mesh_x.clone(),
        material: wall_material_a.clone(),
        transform: Transform::from_xyz(0.0, 2.0, -10.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: wall_mesh_x,
        material: wall_material_b.clone(),
        transform: Transform::from_xyz(0.0, 2.0, 10.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: wall_mesh_z.clone(),
        material: wall_material_b,
        transform: Transform::from_xyz(-10.0, 2.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: wall_mesh_z,
        material: wall_material_a,
        transform: Transform::from_xyz(10.0, 2.0, 0.0),
        ..default()
    });

    let prop_mesh_a = meshes.add(Cuboid::new(2.2, 1.2, 1.0));
    let prop_mesh_b = meshes.add(Cuboid::new(1.4, 0.8, 1.4));
    let prop_material_a = materials.add(Color::srgb(0.12, 0.16, 0.22));
    let prop_material_b = materials.add(Color::srgb(0.08, 0.1, 0.15));
    commands.spawn(PbrBundle {
        mesh: prop_mesh_a,
        material: prop_material_a,
        transform: Transform::from_xyz(-3.5, 0.6, -1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: prop_mesh_b,
        material: prop_material_b,
        transform: Transform::from_xyz(4.0, 0.4, 3.0),
        ..default()
    });

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
            transform: Transform::from_xyz(0.0, 0.9, 0.0),
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
            color: Color::srgb(0.9, 0.95, 1.0),
            illuminance: 1000.0,
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
    for _ in 0..12 {
        let candidate = base + dir * distance;
        if !collides(candidate, radius, obstacles) {
            break;
        }
        distance -= step;
        if distance <= 1.2 {
            return 1.2;
        }
    }
    distance
}

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
