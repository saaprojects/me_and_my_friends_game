use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod tools;

pub use components::Player;

pub struct InvestigatorPlugin;

impl Plugin for InvestigatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::InvestigatorCameraState>()
            .init_resource::<systems::InvestigatorVelocity>()
            .init_resource::<crate::core::InputMap>()
            .init_resource::<crate::core::MovementConfig>()
            .init_resource::<crate::core::CameraConfig>()
            .add_systems(
                Update,
                (
                    systems::investigator_movement_system,
                    tools::handle_equipment_input,
                    tools::update_emf_reading,
                    tools::handle_spiritbox,
                ),
            );
    }
}
