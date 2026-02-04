use crate::prelude::*;

use crate::core::{MenuState, RoleState};
use crate::gameplay::ghost::{GhostMarker, GhostState};
use crate::gameplay::map::components::CollisionWorld;
use crate::gameplay::map::systems::move_with_collisions;

pub fn ghost_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    mut ghost: ResMut<GhostState>,
    control: Res<CameraControl>,
    world: Res<CollisionWorld>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<GhostMarker>)>,
) {
    if menu.open || role.current != Role::Ghost {
        return;
    }

    let delta = time.delta_seconds();
    let speed = 5.2;
    let sprint = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let speed = if sprint { speed * 1.6 } else { speed };

    let forward = Vec3::new(control.yaw.sin(), 0.0, control.yaw.cos());
    let right = Vec3::new(-forward.z, 0.0, forward.x);

    let mut movement = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        movement += forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        movement -= forward;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        movement -= right;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        movement += right;
    }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize() * speed * delta;
    }

    move_with_collisions(
        &mut ghost.position,
        movement,
        0.35,
        world.bounds,
        &world.obstacles,
        false,
    );

    let forward3d = Vec3::new(
        control.yaw.sin() * control.pitch.cos(),
        control.pitch.sin(),
        control.yaw.cos() * control.pitch.cos(),
    );
    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.translation = ghost.position;
        camera.translation.y = 1.6;
        camera.look_at(ghost.position + forward3d, Vec3::Y);
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
