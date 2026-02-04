use crate::prelude::*;

use crate::core::{JournalState, MenuState, RoleState};
use crate::gameplay::map::components::CollisionWorld;
use crate::gameplay::map::systems::{
    avoid_camera_obstacles, clamp_camera_distance, move_with_collisions, shortest_angle,
};
use crate::gameplay::investigator::Player;

pub fn investigator_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    control: Res<CameraControl>,
    world: Res<CollisionWorld>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }

    let delta = time.delta_seconds();
    let speed = 3.6;
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

    if let Ok(mut player) = player_query.get_single_mut() {
        move_with_collisions(
            &mut player.translation,
            movement,
            0.35,
            world.bounds,
            &world.obstacles,
            true,
        );

        let target_yaw = control.yaw + std::f32::consts::PI;
        let diff = shortest_angle(player.rotation.to_euler(EulerRot::YXZ).0, target_yaw);
        let smooth = 0.08;
        player.rotate_y(diff * smooth);

        let radius = 6.3;
        let cam_dir = Vec3::new(
            control.yaw.sin() * control.pitch.cos(),
            control.pitch.sin(),
            control.yaw.cos() * control.pitch.cos(),
        )
        .normalize_or_zero();
        let base_pos = player.translation + Vec3::new(0.0, 1.6, 0.0);
        let dir = -cam_dir;
        let mut t = clamp_camera_distance(base_pos, dir, radius, world.bounds);
        t = avoid_camera_obstacles(base_pos, dir, t, 0.35, &world.obstacles);
        let offset = dir * t;

        if let Ok(mut camera) = camera_query.get_single_mut() {
            camera.translation = base_pos + offset;
            camera.look_at(player.translation + Vec3::new(0.0, 1.2, 0.0), Vec3::Y);
        }
    }
}
