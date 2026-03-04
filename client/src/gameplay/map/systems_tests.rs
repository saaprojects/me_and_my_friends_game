use super::*;

#[test]
fn choose_shell_scene_prefers_three_room_variant_when_available() {
    let chosen = choose_shell_scene(3, true, true, true);
    assert_eq!(chosen, THREE_ROOM_SHELL_SCENE);
}

#[test]
fn choose_shell_scene_prefers_two_room_variant_when_available() {
    let chosen = choose_shell_scene(2, true, false, true);
    assert_eq!(chosen, TWO_ROOM_SHELL_SCENE);
}

#[test]
fn choose_shell_scene_uses_shared_when_specific_variant_missing() {
    let chosen = choose_shell_scene(3, false, false, true);
    assert_eq!(chosen, SHARED_SHELL_SCENE);
}

#[test]
fn choose_shell_scene_falls_back_to_expected_specific_name_when_no_assets_exist() {
    assert_eq!(
        choose_shell_scene(3, false, false, false),
        THREE_ROOM_SHELL_SCENE
    );
    assert_eq!(
        choose_shell_scene(2, false, false, false),
        TWO_ROOM_SHELL_SCENE
    );
}

#[test]
fn room_light_range_stays_within_intended_clamp_bounds() {
    let small = room_light_range(Bounds {
        min_x: 0.0,
        max_x: 1.0,
        min_z: 0.0,
        max_z: 1.0,
    });
    let huge = room_light_range(Bounds {
        min_x: -50.0,
        max_x: 50.0,
        min_z: -50.0,
        max_z: 50.0,
    });
    assert!(small >= 10.0 && small <= 16.0);
    assert!(huge >= 10.0 && huge <= 16.0);
}

#[test]
fn light_flicker_multiplier_is_bounded_for_on_and_off_paths() {
    for elapsed in [0.0, 0.02, 0.07, 0.15, 0.33, 0.61] {
        let on_value = light_flicker_multiplier(elapsed, 1.7, true);
        let off_value = light_flicker_multiplier(elapsed, 1.7, false);
        assert!((0.02..=1.2).contains(&on_value));
        assert!((0.02..=1.2).contains(&off_value));
    }
}

#[test]
fn wall_render_size_adds_axis_overlaps() {
    let input = Vec3::new(0.4, 4.0, 8.6);
    let out = wall_render_size(input);
    assert!((out.x - (input.x + WALL_HORIZONTAL_OVERLAP * 2.0)).abs() < 0.0001);
    assert!((out.z - (input.z + WALL_HORIZONTAL_OVERLAP * 2.0)).abs() < 0.0001);
    assert!((out.y - (input.y + WALL_VERTICAL_OVERLAP * 2.0)).abs() < 0.0001);
}

#[test]
fn wall_render_center_keeps_floor_contact_and_extends_upward() {
    let layout_center = Vec3::new(2.0, 2.0, -5.5);
    let layout_size = Vec3::new(0.4, 4.0, 8.6);
    let render_size = wall_render_size(layout_size);
    let render_center = wall_render_center(layout_center, layout_size, render_size);

    let layout_bottom = layout_center.y - layout_size.y * 0.5;
    let render_bottom = render_center.y - render_size.y * 0.5;
    let render_top = render_center.y + render_size.y * 0.5;
    let layout_top = layout_center.y + layout_size.y * 0.5;

    assert!((render_bottom - layout_bottom).abs() < 0.0001);
    assert!(render_top > layout_top);
}

#[test]
fn wall_scene_transform_accounts_for_bottom_anchored_asset_origin() {
    let center = Vec3::new(2.0, 2.0, -5.5);
    let render_size = Vec3::new(0.4, 4.3, 8.6);
    let transform = wall_scene_transform(center, render_size);

    assert!((transform.translation.x - center.x).abs() < 0.0001);
    assert!((transform.translation.z - center.z).abs() < 0.0001);
    assert!((transform.translation.y - (center.y - render_size.y * 0.5)).abs() < 0.0001);
    assert!((transform.scale.x - (render_size.x / WALL_ASSET_BASE_SIZE.x)).abs() < 0.0001);
    assert!((transform.scale.y - (render_size.y / WALL_ASSET_BASE_SIZE.y)).abs() < 0.0001);
    assert!((transform.scale.z - (render_size.z / WALL_ASSET_BASE_SIZE.z)).abs() < 0.0001);
}

#[test]
fn roof_dimensions_match_outer_wall_extents_for_two_room_layout() {
    let house = HouseLayout::two_room();
    let (roof_x, roof_z, roof_y) = roof_dimensions(&house);

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    let mut wall_top = f32::NEG_INFINITY;
    for wall in &house.walls {
        let hx = wall.size.x * 0.5;
        let hz = wall.size.z * 0.5;
        min_x = min_x.min(wall.translation.x - hx);
        max_x = max_x.max(wall.translation.x + hx);
        min_z = min_z.min(wall.translation.z - hz);
        max_z = max_z.max(wall.translation.z + hz);
        wall_top = wall_top.max(wall.translation.y + wall.size.y * 0.5);
    }

    assert!(roof_x > (max_x - min_x));
    assert!(roof_z > (max_z - min_z));
    assert!((roof_x - (max_x - min_x) - ROOF_OVERHANG_PER_SIDE * 2.0).abs() < 0.0001);
    assert!((roof_z - (max_z - min_z) - ROOF_OVERHANG_PER_SIDE * 2.0).abs() < 0.0001);
    let expected_y = wall_top + ROOF_THICKNESS * 0.5 - ROOF_WALL_OVERLAP_Y;
    assert!((roof_y - expected_y).abs() < 0.0001);
}

#[test]
fn roof_dimensions_match_outer_wall_extents_for_three_room_layout() {
    let house = HouseLayout::three_room();
    let (roof_x, roof_z, roof_y) = roof_dimensions(&house);

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    let mut wall_top = f32::NEG_INFINITY;
    for wall in &house.walls {
        let hx = wall.size.x * 0.5;
        let hz = wall.size.z * 0.5;
        min_x = min_x.min(wall.translation.x - hx);
        max_x = max_x.max(wall.translation.x + hx);
        min_z = min_z.min(wall.translation.z - hz);
        max_z = max_z.max(wall.translation.z + hz);
        wall_top = wall_top.max(wall.translation.y + wall.size.y * 0.5);
    }

    assert!(roof_x > (max_x - min_x));
    assert!(roof_z > (max_z - min_z));
    assert!((roof_x - (max_x - min_x) - ROOF_OVERHANG_PER_SIDE * 2.0).abs() < 0.0001);
    assert!((roof_z - (max_z - min_z) - ROOF_OVERHANG_PER_SIDE * 2.0).abs() < 0.0001);
    let expected_y = wall_top + ROOF_THICKNESS * 0.5 - ROOF_WALL_OVERLAP_Y;
    assert!((roof_y - expected_y).abs() < 0.0001);
}

#[test]
fn rendered_walls_overlap_floor_and_roof_to_avoid_visible_seams() {
    for house in [HouseLayout::two_room(), HouseLayout::three_room()] {
        let (_roof_x, _roof_z, roof_y) = roof_dimensions(&house);
        let roof_bottom = roof_y - ROOF_THICKNESS * 0.5;
        for wall in &house.walls {
            let render_size = wall_render_size(wall.size);
            let render_center = wall_render_center(wall.translation, wall.size, render_size);
            let wall_bottom = render_center.y - render_size.y * 0.5;
            let wall_top = render_center.y + render_size.y * 0.5;
            let original_bottom = wall.translation.y - wall.size.y * 0.5;
            assert!(
                (wall_bottom - original_bottom).abs() < 0.0001,
                "wall bottom should stay flush with floor"
            );
            assert!(
                roof_bottom < wall_top,
                "roof bottom should overlap wall top to prevent seam"
            );
        }
    }
}
