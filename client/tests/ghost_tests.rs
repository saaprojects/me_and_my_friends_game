use bevy::prelude::*;

use client::core::{CameraControl, MenuState, Role, RoleState};
use client::gameplay::ghost::systems::ghost_movement_system;
use client::gameplay::ghost::GhostState;
use client::gameplay::map::components::{Bounds, CollisionWorld, Obstacle};

fn test_world_with_divider() -> CollisionWorld {
    CollisionWorld {
        bounds: Bounds {
            min_x: -9.4,
            max_x: 9.4,
            min_z: -9.4,
            max_z: 9.4,
        },
        obstacles: vec![Obstacle {
            min_x: 1.8,
            max_x: 2.2,
            min_z: -9.4,
            max_z: -1.2,
        },
        Obstacle {
            min_x: 1.8,
            max_x: 2.2,
            min_z: 1.2,
            max_z: 9.4,
        }],
    }
}

fn run_ghost_step(start: Vec3) -> Vec3 {
    let mut app = App::new();
    app.add_systems(Update, ghost_movement_system);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Ghost,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(GhostState { position: start });
    app.insert_resource(CameraControl {
        yaw: std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    });
    app.insert_resource(CollisionWorld {
        bounds: test_world_with_divider().bounds,
        obstacles: test_world_with_divider().obstacles,
    });
    app.insert_resource(Time::<()>::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.world_mut().spawn(Camera3dBundle::default());

    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
    }
    {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(std::time::Duration::from_secs_f32(0.2));
    }

    app.update();
    app.world().resource::<GhostState>().position
}

#[test]
fn ghost_cannot_move_through_divider_wall() {
    let end = run_ghost_step(Vec3::new(1.2, 1.6, 5.8));
    assert!(end.x < 1.8);
}

#[test]
fn ghost_can_move_through_divider_center_doorway() {
    let end = run_ghost_step(Vec3::new(1.2, 1.6, 0.0));
    assert!(end.x > 1.8);
}
