use crate::prelude::*;

use crate::core::{JournalState, MenuState, RoleState, RoleYaw};

pub mod ghost;
pub mod investigator;
pub mod map;
pub mod evidence;
pub mod exorcism;
pub mod debug;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            map::MapPlugin,
            ghost::GhostPlugin,
            investigator::InvestigatorPlugin,
            exorcism::ExorcismPlugin,
        ))
            .init_resource::<evidence::EvidenceTuning>()
            .insert_resource(debug::DebugOverlayState { enabled: true })
            .add_systems(Startup, debug::spawn_debug_overlay)
            .add_systems(
                Update,
                (
                    handle_role_toggle,
                    update_mouse_look,
                    debug::toggle_debug_overlay,
                    debug::sync_debug_overlay,
                ),
            );
    }
}

fn handle_role_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    mut role: ResMut<RoleState>,
    mut role_yaw: ResMut<RoleYaw>,
    mut control: ResMut<CameraControl>,
) {
    if menu.open || journal.open {
        return;
    }
    if keys.just_pressed(KeyCode::Tab) || keys.just_pressed(KeyCode::KeyT) {
        role.current = match role.current {
            Role::Ghost => Role::Investigator,
            Role::Investigator => Role::Ghost,
        };
        set_default_camera(role.current, &mut control, &mut role_yaw);
    }
}

fn update_mouse_look(
    mut motion_events: EventReader<MouseMotion>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    mut control: ResMut<CameraControl>,
    mut role_yaw: ResMut<RoleYaw>,
) {
    if menu.open || journal.open {
        return;
    }
    let sensitivity = 0.0022;
    if motion_events.is_empty() {
        return;
    }

    for event in motion_events.read() {
        control.yaw -= event.delta.x * sensitivity;
        control.pitch -= event.delta.y * sensitivity;
    }

    if role.current == Role::Ghost {
        control.pitch = control.pitch.clamp(-0.6, 0.6);
    } else {
        control.pitch = control.pitch.clamp(-0.25, 0.4);
    }

    if control.yaw > std::f32::consts::PI {
        control.yaw -= std::f32::consts::TAU;
    }
    if control.yaw < -std::f32::consts::PI {
        control.yaw += std::f32::consts::TAU;
    }

    match role.current {
        Role::Ghost => role_yaw.ghost = control.yaw,
        Role::Investigator => role_yaw.investigator = control.yaw,
    }
}
