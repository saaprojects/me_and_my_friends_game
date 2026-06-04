use crate::prelude::*;

use crate::core::health::{
    spawn_health_thread, update_health, update_window_title, HealthChannel, HealthState,
};
use crate::core::{CameraConfig, InputMap, MovementConfig};
use crate::gameplay::{
    evidence::EvidenceTuning,
    ghost::GhostState,
    investigator::tools::{EquipmentState, EvidenceState},
    map::{HouseLayout, HouseLayoutSelection},
    GameplayPlugin,
};
use crate::ui::UiPlugin;

pub fn run() {
    let rx = spawn_health_thread();
    let initial_house = HouseLayout::two_room();
    let mut rng = crate::core::GameRng::default();

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
            position: initial_house.random_ghost_spawn(&mut rng),
        })
        .insert_resource(HouseLayoutSelection::default())
        .insert_resource(EquipmentState::default())
        .insert_resource(InputMap::default())
        .insert_resource(MovementConfig::default())
        .insert_resource(CameraConfig::default())
        .insert_resource(rng)
        .insert_resource(GhostTypeState {
            selected: GhostType::Spirit,
            active: GhostType::Spirit,
        })
        .insert_resource(crate::core::SessionState { started: false })
        .insert_resource(crate::core::ResolutionState::default())
        .insert_resource(EvidenceState::default())
        .insert_resource(EvidenceTuning::default())
        .insert_resource(HealthState {
            status: "loading".to_string(),
        })
        .insert_resource(HealthChannel { rx })
        .insert_resource(AmbientLight {
            color: Color::srgb(0.72, 0.76, 0.92),
            brightness: 0.22,
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
