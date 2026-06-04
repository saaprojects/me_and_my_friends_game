use crate::prelude::*;

use crate::core::{InputMap, MenuState, MovementConfig, RoleState};
use crate::gameplay::ghost::{GhostBobState, GhostMarker, GhostState, GhostVelocity};
use crate::gameplay::map::components::CollisionWorld;
use crate::gameplay::map::systems::move_with_collisions;
use crate::gameplay::{is_sprinting, read_movement_direction};

pub fn ghost_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    input: Res<InputMap>,
    movement_cfg: Res<MovementConfig>,
    mut ghost: ResMut<GhostState>,
    mut ghost_vel: ResMut<GhostVelocity>,
    mut bob: ResMut<GhostBobState>,
    control: Res<CameraControl>,
    world: Res<CollisionWorld>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<GhostMarker>)>,
) {
    if menu.open || role.current != Role::Ghost {
        return;
    }

    let delta = time.delta_seconds();
    let sprinting = is_sprinting(&keys, &input);
    let base_speed = movement_cfg.ghost_speed;
    let speed = if sprinting {
        base_speed * movement_cfg.ghost_sprint_mul
    } else {
        base_speed
    };

    let dir = read_movement_direction(&keys, &control, &input);
    let moving = dir.length_squared() > 0.01;

    let target_vel = dir * speed;
    let accel = if moving { 10.0 } else { 8.0 };
    ghost_vel.0 = ghost_vel.0.lerp(target_vel, (delta * accel).min(1.0));

    move_with_collisions(
        &mut ghost.position,
        ghost_vel.0 * delta,
        0.35,
        world.bounds,
        &world.obstacles,
        true,
    );

    let bob_speed = if sprinting { 8.0 } else { 5.5 };
    bob.magnitude = if moving {
        (bob.magnitude + delta * 5.0).min(1.0)
    } else {
        (bob.magnitude - delta * 6.0).max(0.0)
    };
    if moving {
        bob.phase += delta * bob_speed;
    }
    let bob_y = bob.phase.sin() * 0.055 * bob.magnitude
        + (bob.phase * 2.0).sin().abs() * 0.018 * bob.magnitude;

    let forward3d = Vec3::new(
        control.yaw.sin() * control.pitch.cos(),
        control.pitch.sin(),
        control.yaw.cos() * control.pitch.cos(),
    );
    if let Ok(mut camera) = camera_query.get_single_mut() {
        let eye = Vec3::new(ghost.position.x, 1.6 + bob_y, ghost.position.z);
        camera.translation = eye;
        camera.look_at(eye + forward3d, Vec3::Y);
    }
}

pub fn sync_ghost_marker(
    ghost: Res<GhostState>,
    mut markers: Query<&mut Transform, With<GhostMarker>>,
) {
    let Ok(mut marker) = markers.get_single_mut() else {
        return;
    };
    marker.translation = Vec3::new(ghost.position.x, 1.2, ghost.position.z);
}

#[cfg(test)]
#[path = "systems_tests.rs"]
mod systems_tests;
