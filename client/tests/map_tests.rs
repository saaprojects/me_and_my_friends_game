use bevy::prelude::Vec3;
use client::gameplay::map::HouseLayout;
use client::gameplay::map::components::HouseLayoutSelection;
use client::gameplay::map::systems::{
    avoid_camera_obstacles, collides, ghost_spawn_positions, investigator_spawn_position,
    random_ghost_spawn_position, room_id, room_id_in_house, shortest_angle,
};
use client::gameplay::map::components::Obstacle;

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

#[test]
fn investigator_spawns_in_main_room() {
    let spawn = investigator_spawn_position();
    assert_eq!(spawn, Vec3::new(-6.0, 0.9, 0.0));
}

#[test]
fn random_ghost_spawn_is_from_allowed_positions() {
    let allowed = ghost_spawn_positions();
    let spawn = random_ghost_spawn_position();
    assert!(allowed.contains(&spawn));
}

#[test]
fn divider_has_center_doorway_gap_only() {
    let divider = vec![
        Obstacle {
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
        },
    ];

    assert!(collides(Vec3::new(2.0, 0.0, 5.8), 0.35, &divider));
    assert!(!collides(Vec3::new(2.0, 0.0, 0.0), 0.35, &divider));
}

#[test]
fn house_layout_defines_rooms_and_spawn_zones() {
    let layout = HouseLayout::two_room();

    assert_eq!(layout.rooms.len(), 2);
    assert_eq!(layout.rooms[0].name, "Main Room");
    assert_eq!(layout.rooms[1].name, "Side Room");

    assert!(layout.rooms[0].bounds.contains_xz(layout.investigator_spawn));
    assert!(layout
        .ghost_spawns
        .iter()
        .any(|spawn| layout.rooms[0].bounds.contains_xz(*spawn)));
    assert!(layout
        .ghost_spawns
        .iter()
        .any(|spawn| layout.rooms[1].bounds.contains_xz(*spawn)));
    assert_eq!(layout.exorcism.spirit_anchors.len(), 3);
    assert_eq!(layout.exorcism.banshee_anchors.len(), 3);
    assert_eq!(layout.exorcism.onryo_cursed_positions.len(), 3);
    assert_eq!(layout.exorcism.onryo_ritual_positions.len(), 3);
}

#[test]
fn house_layout_contains_wall_visuals_for_outer_and_divider_walls() {
    let layout = HouseLayout::two_room();

    assert_eq!(layout.walls.len(), 6);
    assert!(layout
        .walls
        .iter()
        .any(|wall| wall.translation == Vec3::new(2.0, 2.0, -5.4) && wall.size.z >= 8.3));
    assert!(layout
        .walls
        .iter()
        .any(|wall| wall.translation == Vec3::new(2.0, 2.0, 5.4) && wall.size.z >= 8.3));
}

#[test]
fn room_lookup_uses_house_layout_rooms() {
    let layout = HouseLayout::two_room();

    assert_eq!(room_id_in_house(&layout, Vec3::new(-5.0, 0.0, 0.0)), Some(0));
    assert_eq!(room_id_in_house(&layout, Vec3::new(5.0, 0.0, 0.0)), Some(1));
    assert_eq!(room_id_in_house(&layout, Vec3::new(2.0, 0.0, 5.0)), None);
}

#[test]
fn three_room_layout_adds_a_third_room_with_intentional_doorways() {
    let layout = HouseLayout::three_room();

    assert_eq!(layout.rooms.len(), 3);
    assert_eq!(layout.rooms[0].name, "Main Room");
    assert_eq!(layout.rooms[1].name, "Upper Room");
    assert_eq!(layout.rooms[2].name, "Side Room");
    assert_eq!(layout.walls.len(), 8);
    assert_eq!(layout.ghost_spawns.len(), 3);

    assert_eq!(room_id_in_house(&layout, Vec3::new(-5.0, 0.0, -6.0)), Some(0));
    assert_eq!(room_id_in_house(&layout, Vec3::new(-5.0, 0.0, 5.0)), Some(1));
    assert_eq!(room_id_in_house(&layout, Vec3::new(5.0, 0.0, 0.0)), Some(2));

    assert!(collides(Vec3::new(-5.7, 0.0, -2.0), 0.35, &layout.obstacles));
    assert!(!collides(Vec3::new(-1.0, 0.0, -2.0), 0.35, &layout.obstacles));
    assert!(!collides(Vec3::new(2.0, 0.0, 0.0), 0.35, &layout.obstacles));
    assert!(layout
        .walls
        .iter()
        .any(|wall| wall.translation == Vec3::new(-5.7, 2.0, -2.0) && wall.size.x >= 7.7));
    assert!(layout
        .walls
        .iter()
        .any(|wall| wall.translation == Vec3::new(1.1, 2.0, -2.0) && wall.size.x >= 1.7));
}

#[test]
fn house_layout_selection_defaults_to_two_room() {
    let selection = HouseLayoutSelection::default();
    assert_eq!(selection.selected_kind, client::gameplay::map::HouseLayoutKind::TwoRoom);
    assert_eq!(selection.active_kind, client::gameplay::map::HouseLayoutKind::TwoRoom);
}

#[test]
fn camera_avoidance_does_not_jump_through_thin_wall() {
    let base = Vec3::new(0.0, 1.6, 0.0);
    let dir = Vec3::X;
    let wall = [Obstacle {
        min_x: 1.8,
        max_x: 2.2,
        min_z: -2.0,
        max_z: 2.0,
    }];

    let distance = avoid_camera_obstacles(base, dir, 5.0, 0.35, &wall);
    let candidate = base + dir * distance;

    assert!(candidate.x < 1.8);
}
