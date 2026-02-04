use crate::prelude::*;

use crate::core::health::{spawn_health_thread, HealthChannel, HealthState, update_health, update_window_title};
use crate::gameplay::{
    evidence::EvidenceTuning,
    ghost::GhostState,
    investigator::tools::{EvidenceState, EquipmentState},
    GameplayPlugin,
};
use crate::ui::UiPlugin;

pub fn run() {
    let rx = spawn_health_thread();

    App::new()
        .insert_resource(RoleState {
            current: crate::core::Role::Ghost,
        })
        .insert_resource(MenuState {
            open: true,
            selected_role: crate::core::Role::Ghost,
        })
        .insert_resource(crate::core::MenuFlowState {
            screen: crate::core::MenuScreen::Start,
        })
        .insert_resource(crate::core::JournalState { open: false })
        .insert_resource(RoleYaw {
            ghost: 0.0,
            investigator: 0.0,
        })
        .insert_resource(CameraControl {
            yaw: 0.0,
            pitch: 0.12,
        })
        .insert_resource(GhostState {
            position: Vec3::new(0.0, 1.6, 5.0),
        })
        .insert_resource(EquipmentState {
            active: crate::core::Equipment::Emf,
            emf_level: 0,
            emf_dwell: 0.0,
            emf_smoothed: 0.0,
            emf_evidence_latch: 0.0,
            spiritbox_message: "Silence...".to_string(),
            spiritbox_cooldown: 0.0,
        })
        .insert_resource(GhostTypeState {
            selected: GhostType::Spirit,
            active: GhostType::Spirit,
        })
        .insert_resource(crate::core::SessionState { started: false })
        .insert_resource(EvidenceState::default())
        .insert_resource(EvidenceTuning::default())
        .insert_resource(HealthState {
            status: "loading".to_string(),
        })
        .insert_resource(HealthChannel { rx })
        .insert_resource(AmbientLight {
            color: Color::srgb(0.7, 0.75, 0.9),
            brightness: 0.45,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Me & My Friends".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameplayPlugin, UiPlugin))
        .add_systems(Update, (update_health, update_window_title))
        .run();
}
