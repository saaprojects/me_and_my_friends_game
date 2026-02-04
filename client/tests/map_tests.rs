use bevy::prelude::Vec3;
use client::gameplay::map::systems::{room_id, shortest_angle};

#[test]
fn room_id_quadrants() {
    assert_eq!(room_id(Vec3::new(-1.0, 0.0, -1.0)), 0);
    assert_eq!(room_id(Vec3::new(1.0, 0.0, -1.0)), 2);
    assert_eq!(room_id(Vec3::new(-1.0, 0.0, 1.0)), 1);
    assert_eq!(room_id(Vec3::new(1.0, 0.0, 1.0)), 3);
}

#[test]
fn shortest_angle_wraps() {
    let diff = shortest_angle(3.0, -3.0);
    assert!(diff.abs() < 0.4);
}
