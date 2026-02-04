use crate::prelude::*;

use crate::core::{JournalState, MenuState, Role, RoleState};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::investigator::Player;
use crate::gameplay::evidence::EvidenceTuning;

#[derive(Resource)]
pub struct DebugOverlayState {
    pub enabled: bool,
}

#[derive(Component)]
pub struct DebugToolBubble;

#[derive(Component)]
pub struct DebugGhostBubble;

#[derive(Component)]
pub struct DebugFacingRay;

pub fn spawn_debug_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tuning: Res<EvidenceTuning>,
) {
    let tool_mesh = meshes
        .add(Sphere::new(tuning.tool_bubble_radius).mesh().uv(24, 18));
    let ghost_mesh = meshes
        .add(Sphere::new(tuning.ghost_influence_radius).mesh().uv(24, 18));
    let ray_mesh = meshes.add(Cuboid::new(0.04, 0.04, 2.0));

    let tool_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.6, 1.0, 0.18),
        emissive: Color::srgb(0.05, 0.12, 0.2).into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ghost_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.35, 0.35, 0.18),
        emissive: Color::srgb(0.2, 0.05, 0.05).into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ray_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.9, 0.2, 0.7),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: tool_mesh,
            material: tool_material,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        DebugToolBubble,
    ));

    commands.spawn((
        PbrBundle {
            mesh: ghost_mesh,
            material: ghost_material,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        DebugGhostBubble,
    ));

    commands.spawn((
        PbrBundle {
            mesh: ray_mesh,
            material: ray_material,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        DebugFacingRay,
    ));
}

pub fn toggle_debug_overlay(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugOverlayState>,
) {
    if keys.just_pressed(KeyCode::F3) {
        debug.enabled = !debug.enabled;
    }
}

pub fn sync_debug_overlay(
    debug: Res<DebugOverlayState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    role: Res<RoleState>,
    ghost: Res<GhostState>,
    control: Res<CameraControl>,
    camera: Query<
        &Transform,
        (
            With<Camera>,
            Without<DebugToolBubble>,
            Without<DebugGhostBubble>,
            Without<DebugFacingRay>,
        ),
    >,
    player: Query<
        &Transform,
        (
            With<Player>,
            Without<DebugToolBubble>,
            Without<DebugGhostBubble>,
            Without<DebugFacingRay>,
        ),
    >,
    mut overlays: ParamSet<(
        Query<(&mut Transform, &mut Visibility), With<DebugToolBubble>>,
        Query<(&mut Transform, &mut Visibility), With<DebugGhostBubble>>,
        Query<(&mut Transform, &mut Visibility), With<DebugFacingRay>>,
    )>,
) {
    let visible = debug.enabled && !menu.open && !journal.open && role.current == Role::Investigator;
    let visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    if let Ok((mut transform, mut vis)) = overlays.p0().get_single_mut() {
        transform.translation = player_transform.translation;
        *vis = visibility;
    }

    if let Ok((mut transform, mut vis)) = overlays.p1().get_single_mut() {
        transform.translation = ghost.position;
        *vis = visibility;
    }

    if let Ok((mut transform, mut vis)) = overlays.p2().get_single_mut() {
        let (forward, yaw) = if let Ok(cam_transform) = camera.get_single() {
            let cam_forward = cam_transform.forward();
            let flat_forward = Vec3::new(cam_forward.x, 0.0, cam_forward.z).normalize_or_zero();
            let yaw = flat_forward.x.atan2(flat_forward.z);
            (flat_forward, yaw)
        } else {
            let yaw = control.yaw;
            let forward = Vec3::new(yaw.sin(), 0.0, yaw.cos()).normalize_or_zero();
            (forward, yaw)
        };

        transform.translation =
            player_transform.translation + forward * 1.0 + Vec3::new(0.0, 1.2, 0.0);
        transform.rotation = Quat::from_rotation_y(yaw);
        *vis = visibility;
    }
}
