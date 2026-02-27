use crate::prelude::*;

pub mod components;
pub mod systems;

pub use components::{HouseLayout, HouseLayoutKind, HouseLayoutSelection};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let house = components::HouseLayout::two_room();
        app.init_resource::<components::HouseLayoutSelection>()
        .insert_resource(house.collision_world())
        .insert_resource(house)
        .add_systems(Startup, systems::setup_scene)
        .add_systems(
            Update,
            (
                systems::sync_layout_walls,
                systems::sync_room_light_visuals,
                systems::animate_room_light_flicker,
            )
                .chain(),
        );
    }
}
