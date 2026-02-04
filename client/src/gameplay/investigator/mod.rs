use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod tools;

pub use components::Player;

pub struct InvestigatorPlugin;

impl Plugin for InvestigatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
