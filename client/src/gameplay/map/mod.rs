use crate::prelude::*;

pub mod camera;
pub mod collision;
pub mod components;
pub mod rooms;
pub mod systems;

pub use components::{ExorcismPlacement, HouseLayout, HouseLayoutKind, HouseLayoutSelection};

#[cfg(test)]
mod map_tests;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let house = components::HouseLayout::two_room();
        let placement = components::ExorcismPlacement::from_layout(&house.exorcism);
        app.init_resource::<components::HouseLayoutSelection>()
            .insert_resource(house.collision_world())
            .insert_resource(house)
            .insert_resource(placement)
            .add_systems(Startup, systems::setup_scene)
            .add_systems(
                Update,
                (
                    sync_exorcism_placement,
                    systems::sync_layout_walls,
                    systems::sync_room_light_visuals,
                    systems::animate_room_light_flicker,
                )
                    .chain(),
            );
    }
}

fn sync_exorcism_placement(
    house: Res<components::HouseLayout>,
    mut placement: ResMut<components::ExorcismPlacement>,
) {
    if !house.is_changed() {
        return;
    }
    *placement = components::ExorcismPlacement::from_layout(&house.exorcism);
}
