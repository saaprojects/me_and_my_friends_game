use crate::prelude::*;

use crate::core::{CameraConfig, InputMap, JournalState, MenuState, MovementConfig, RoleState};
use crate::gameplay::investigator::Player;
use crate::gameplay::map::components::CollisionWorld;
use crate::gameplay::map::systems::{
    avoid_camera_obstacles, clamp_camera_distance, move_with_collisions, shortest_angle,
};
use crate::gameplay::{is_sprinting, read_movement_direction};

#[derive(Resource)]
pub struct InvestigatorCameraState {
    distance: f32,
}

impl Default for InvestigatorCameraState {
    fn default() -> Self {
        Self { distance: 4.8 }
    }
}

#[derive(Resource, Default)]
pub struct InvestigatorVelocity(pub Vec3);

pub fn investigator_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    input: Res<InputMap>,
    movement_cfg: Res<MovementConfig>,
    camera_cfg: Res<CameraConfig>,
    control: Res<CameraControl>,
    world: Res<CollisionWorld>,
    camera_state: Option<ResMut<InvestigatorCameraState>>,
    mut inv_vel: ResMut<InvestigatorVelocity>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }

    let delta = time.delta_seconds();
    let base_speed = movement_cfg.investigator_speed;
    let sprinting = is_sprinting(&keys, &input);
    let speed = if sprinting {
        base_speed * movement_cfg.investigator_sprint_mul
    } else {
        base_speed
    };

    let dir = read_movement_direction(&keys, &control, &input);
    let moving = dir.length_squared() > 0.01;
    let target_vel = dir * speed;
    let accel = if moving { 10.0 } else { 8.0 };
    inv_vel.0 = inv_vel.0.lerp(target_vel, (delta * accel).min(1.0));
    let movement = inv_vel.0 * delta;

    if let Ok(mut player) = player_query.get_single_mut() {
        move_with_collisions(
            &mut player.translation,
            movement,
            0.35,
            world.bounds,
            &world.obstacles,
            true,
        );

        let target_yaw = control.yaw;
        let diff = shortest_angle(player.rotation.to_euler(EulerRot::YXZ).0, target_yaw);
        let smooth = 0.08;
        player.rotate_y(diff * smooth);

        let cam_dir = Vec3::new(
            control.yaw.sin() * control.pitch.cos(),
            control.pitch.sin(),
            control.yaw.cos() * control.pitch.cos(),
        )
        .normalize_or_zero();
        let base_pos = player.translation + Vec3::new(0.0, 1.6, 0.0);
        let dir = -cam_dir;
        let mut t = clamp_camera_distance(base_pos, dir, camera_cfg.radius, world.bounds);
        t = avoid_camera_obstacles(base_pos, dir, t, 0.35, &world.obstacles);

        let distance = if let Some(mut state) = camera_state {
            let blend = 1.0 - (-delta * camera_cfg.smooth_rate).exp();
            state.distance += (t - state.distance) * blend;
            state.distance = state
                .distance
                .clamp(camera_cfg.min_distance, camera_cfg.radius);
            state.distance
        } else {
            t
        };
        let offset = dir * distance;

        if let Ok(mut camera) = camera_query.get_single_mut() {
            camera.translation = base_pos + offset;
            camera.look_at(player.translation + Vec3::new(0.0, 1.2, 0.0), Vec3::Y);
        }
    }
}

#[cfg(test)]
#[path = "systems_tests.rs"]
mod systems_tests;
