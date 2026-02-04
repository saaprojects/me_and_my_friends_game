use bevy::prelude::*;

use client::core::{CameraControl, JournalState, MenuState, Role, RoleState};
use client::gameplay::investigator::systems::investigator_movement_system;
use client::gameplay::investigator::Player;
use client::gameplay::map::components::{Bounds, CollisionWorld};

#[test]
fn investigator_faces_forward_with_pi_offset() {
    let mut app = App::new();
    app.add_systems(Update, investigator_movement_system);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(CollisionWorld {
        bounds: Bounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
        },
        obstacles: Vec::new(),
    });
    app.insert_resource(Time::<()>::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());

    let player = app
        .world_mut()
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            Player,
        ))
        .id();

    app.world_mut().spawn(Camera3dBundle::default());

    {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(std::time::Duration::from_secs_f32(0.016));
    }
    app.update();

    let transform = app.world().entity(player).get::<Transform>().unwrap();
    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
    assert!(yaw.abs() > 0.1);
}
