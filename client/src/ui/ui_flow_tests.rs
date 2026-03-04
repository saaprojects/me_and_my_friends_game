use bevy::prelude::*;
use bevy::window::{Cursor, PrimaryWindow};

use crate::core::{
    Equipment, GhostType, GhostTypeState, JournalState, MenuFlowState, MenuScreen, MenuState, Role,
    RoleState, SessionState,
};
use crate::core::CameraControl;
use crate::gameplay::evidence::EvidenceTuning;
use crate::gameplay::exorcism::{
    ExorcismPlugin, ExorcismStatus, InvestigationState, PuzzleSpawned, SpiritMarker,
};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::investigator::tools::{update_emf_reading, EquipmentState, EvidenceState};
use crate::gameplay::map::{HouseLayout, HouseLayoutKind, HouseLayoutSelection};
use crate::gameplay::map::components::{Bounds, CollisionWorld, RoomZone};
use crate::ui::hud;
use crate::ui::*;

#[test]
fn journal_confirm_updates_investigation_state() {
    let mut app = App::new();
    app.add_systems(Update, hud::handle_journal_interactions);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: true });
    app.insert_resource(InvestigationState::default());
    app.insert_resource(PuzzleSpawned(true));

    let select = app
        .world_mut()
        .spawn((Button, Interaction::None, JournalSelectBansheeButton))
        .id();
    let confirm = app
        .world_mut()
        .spawn((Button, Interaction::None, JournalConfirmButton))
        .id();

    app.world_mut().entity_mut(select).insert(Interaction::Pressed);
    app.update();

    app.world_mut().entity_mut(confirm).insert(Interaction::Pressed);
    app.update();

    let investigation = app.world().resource::<InvestigationState>();
    assert!(matches!(investigation.guess, Some(GhostType::Banshee)));
    assert!(investigation.confirmed);

    let spawned = app.world().resource::<PuzzleSpawned>();
    assert!(!spawned.0);
}

#[test]
fn journal_visibility_hides_after_confirm() {
    let mut app = App::new();
    app.add_systems(Update, hud::sync_journal_visibility);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: true });
    app.insert_resource(InvestigationState {
        guess: Some(GhostType::Spirit),
        confirmed: true,
    });

    let entity = app
        .world_mut()
        .spawn((
            Button,
            Visibility::Visible,
            JournalSelectSpiritButton,
        ))
        .id();

    app.update();

    let visibility = app.world().entity(entity).get::<Visibility>().unwrap();
    assert_eq!(*visibility, Visibility::Hidden);
}

#[test]
fn journal_visibility_shows_before_confirm() {
    let mut app = App::new();
    app.add_systems(Update, hud::sync_journal_visibility);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: true });
    app.insert_resource(InvestigationState::default());

    let entity = app
        .world_mut()
        .spawn((
            Button,
            Visibility::Hidden,
            JournalSelectSpiritButton,
        ))
        .id();

    app.update();

    let visibility = app.world().entity(entity).get::<Visibility>().unwrap();
    assert_eq!(*visibility, Visibility::Visible);
}

#[test]
fn puzzle_spawns_only_after_confirm() {
    let mut app = App::new();
    app.add_plugins(ExorcismPlugin);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(GhostState {
        position: Vec3::ZERO,
    });
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.insert_resource(Time::<()>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());

    app.update();
    let spawned = app.world().resource::<PuzzleSpawned>();
    assert!(!spawned.0);

    {
        let mut investigation = app.world_mut().resource_mut::<InvestigationState>();
        investigation.guess = Some(GhostType::Spirit);
        investigation.confirmed = true;
    }

    app.update();
    let spawned = app.world().resource::<PuzzleSpawned>();
    assert!(spawned.0);
}

#[test]
fn puzzle_title_waits_for_confirmation() {
    let mut app = App::new();
    app.add_systems(Update, hud::sync_hud_text);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(InvestigationState::default());
    app.insert_resource(JournalState { open: false });
    app.insert_resource(ExorcismStatus {
        state: crate::gameplay::exorcism::ExorcismState::Inactive,
        progress: 0.0,
        stage: 0,
        stacks: 0.0,
        max_stacks: 0.0,
    });
    app.insert_resource(crate::gameplay::exorcism::tables::ExorcismTables::default());
    app.insert_resource(EquipmentState {
        active: Equipment::Emf,
        emf_level: 0,
        emf_dwell: 0.0,
        emf_smoothed: 0.0,
        emf_evidence_latch: 0.0,
        spiritbox_message: "Silence...".to_string(),
        spiritbox_cooldown: 0.0,
    });
    app.insert_resource(crate::gameplay::investigator::tools::EvidenceState::default());
    app.insert_resource(EvidenceTuning::default());

    let entity = app
        .world_mut()
        .spawn((
            TextBundle::from_section("Placeholder", TextStyle::default()),
            PuzzleTitleText,
        ))
        .id();

    app.update();

    let text = app.world().entity(entity).get::<Text>().unwrap();
    assert_eq!(text.sections[0].value, "Puzzle: Awaiting Confirmation");
}

#[test]
fn start_screen_visible_when_menu_open() {
    let mut app = App::new();
    app.add_systems(Update, crate::ui::lobby::sync_start_screen_visibility);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::Start,
    });

    let root = app
        .world_mut()
        .spawn((Visibility::Hidden, StartScreenRoot))
        .id();

    app.update();

    let visibility = app.world().entity(root).get::<Visibility>().unwrap();
    assert_eq!(*visibility, Visibility::Visible);
}

#[test]
fn role_select_visible_when_screen_active() {
    let mut app = App::new();
    app.add_systems(Update, crate::ui::lobby::sync_role_select_visibility);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::RoleSelect,
    });

    let root = app
        .world_mut()
        .spawn((Visibility::Hidden, RoleSelectRoot))
        .id();

    app.update();

    let visibility = app.world().entity(root).get::<Visibility>().unwrap();
    assert_eq!(*visibility, Visibility::Visible);
}

#[test]
fn start_button_moves_to_role_select() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::Start,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: false });

    let button = app
        .world_mut()
        .spawn((
            Button,
            Interaction::None,
            BackgroundColor(Color::BLACK),
            StartScreenButton,
        ))
        .id();

    app.world_mut()
        .entity_mut(button)
        .insert(Interaction::Pressed);
    app.update();

    let flow = app.world().resource::<MenuFlowState>();
    assert!(matches!(flow.screen, MenuScreen::RoleSelect));
}

#[test]
fn room_count_selection_changes_only_on_ghost_detail_screen() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::InvestigatorDetails,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(HouseLayoutSelection::default());

    let button = app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        BackgroundColor(Color::BLACK),
        ThreeRoomCountButton,
    ));
    let _button_id = button.id();

    app.update();
    let selection = app.world().resource::<HouseLayoutSelection>();
    assert_eq!(selection.selected_kind, HouseLayoutKind::TwoRoom);

    app.world_mut()
        .resource_mut::<MenuFlowState>()
        .screen = MenuScreen::GhostDetails;
    app.world_mut()
        .entity_mut(_button_id)
        .insert(Interaction::None);
    app.update();
    app.world_mut()
        .entity_mut(_button_id)
        .insert(Interaction::Pressed);
    app.update();

    let selection = app.world().resource::<HouseLayoutSelection>();
    assert_eq!(selection.selected_kind, HouseLayoutKind::ThreeRoom);
    assert_eq!(selection.active_kind, HouseLayoutKind::TwoRoom);
}

#[test]
fn begin_haunt_applies_selected_room_count_layout() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::GhostDetails,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(HouseLayoutSelection::default());
    app.insert_resource(HouseLayout::two_room());
    app.insert_resource(HouseLayout::two_room().collision_world());
    app.insert_resource(GhostState {
        position: Vec3::new(0.0, 1.6, 0.0),
    });
    app.world_mut().spawn((
        Transform::from_xyz(0.0, 0.9, 0.0),
        GlobalTransform::default(),
        crate::gameplay::investigator::Player,
    ));

    let room_button = app
        .world_mut()
        .spawn((
            Button,
            Interaction::Pressed,
            BackgroundColor(Color::BLACK),
            ThreeRoomCountButton,
        ))
        .id();
    app.update();

    app.world_mut().entity_mut(room_button).insert(Interaction::None);
    app.update();

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        BackgroundColor(Color::BLACK),
        BeginHauntButton,
    ));
    app.update();

    let selection = app.world().resource::<HouseLayoutSelection>();
    assert_eq!(selection.selected_kind, HouseLayoutKind::ThreeRoom);
    assert_eq!(selection.active_kind, HouseLayoutKind::ThreeRoom);

    let layout = app.world().resource::<HouseLayout>();
    assert_eq!(layout.rooms.len(), 3);
    assert!(layout.obstacles.iter().any(|o| {
        o.min_x == -9.4 && o.max_x == -1.8 && o.min_z == -2.2 && o.max_z == -1.8
    }));
    assert!(layout.obstacles.iter().any(|o| {
        o.min_x == 0.2 && o.max_x == 1.8 && o.min_z == -2.2 && o.max_z == -1.8
    }));

    let collision = app.world().resource::<CollisionWorld>();
    assert_eq!(collision.obstacles.len(), layout.obstacles.len());
    assert!(collision.obstacles.iter().any(|o| {
        o.min_x == 1.8 && o.max_x == 2.2 && o.min_z == -9.4 && o.max_z == -1.2
    }));
    assert!(collision.obstacles.iter().any(|o| {
        o.min_x == 1.8 && o.max_x == 2.2 && o.min_z == 1.2 && o.max_z == 9.4
    }));
}

#[test]
fn begin_investigation_applies_selected_room_count_layout() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::GhostDetails,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(HouseLayoutSelection::default());
    app.insert_resource(HouseLayout::two_room());
    app.insert_resource(HouseLayout::two_room().collision_world());
    app.insert_resource(GhostState {
        position: Vec3::new(0.0, 1.6, 0.0),
    });
    app.world_mut().spawn((
        Transform::from_xyz(0.0, 0.9, 0.0),
        GlobalTransform::default(),
        crate::gameplay::investigator::Player,
    ));

    let room_button = app
        .world_mut()
        .spawn((
            Button,
            Interaction::Pressed,
            BackgroundColor(Color::BLACK),
            ThreeRoomCountButton,
        ))
        .id();
    app.update();

    app.world_mut()
        .resource_mut::<MenuFlowState>()
        .screen = MenuScreen::InvestigatorDetails;
    app.world_mut().entity_mut(room_button).insert(Interaction::None);
    app.update();

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        BackgroundColor(Color::BLACK),
        BeginInvestigationButton,
    ));
    app.update();

    let selection = app.world().resource::<HouseLayoutSelection>();
    assert_eq!(selection.selected_kind, HouseLayoutKind::ThreeRoom);
    assert_eq!(selection.active_kind, HouseLayoutKind::ThreeRoom);
    assert_eq!(app.world().resource::<HouseLayout>().rooms.len(), 3);
}

#[test]
fn begin_investigation_resets_player_and_ghost_spawns() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::InvestigatorDetails,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Banshee,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 1.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: true });
    app.insert_resource(GhostState {
        position: Vec3::new(0.0, 1.6, 0.0),
    });

    let player = app
        .world_mut()
        .spawn((
            Transform::from_xyz(8.0, 0.9, 8.0),
            GlobalTransform::default(),
            crate::gameplay::investigator::Player,
        ))
        .id();

    let button = app
        .world_mut()
        .spawn((
            Button,
            Interaction::Pressed,
            BackgroundColor(Color::BLACK),
            BeginInvestigationButton,
        ))
        .id();

    app.update();

    let player_transform = app.world().entity(player).get::<Transform>().unwrap();
    assert_eq!(
        player_transform.translation,
        crate::gameplay::map::systems::investigator_spawn_position()
    );

    let ghost = app.world().resource::<GhostState>();
    let allowed = crate::gameplay::map::systems::ghost_spawn_positions();
    assert!(allowed.contains(&ghost.position));

    let role = app.world().resource::<RoleState>();
    assert!(matches!(role.current, Role::Investigator));

    let journal = app.world().resource::<JournalState>();
    assert!(!journal.open);

    let _button_color = app.world().entity(button).get::<BackgroundColor>().unwrap();
}

#[test]
fn begin_investigation_uses_house_layout_spawn_metadata_when_present() {
    let mut app = App::new();
    app.add_event::<AppExit>();
    app.add_systems(Update, crate::ui::lobby::handle_menu_interactions);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::InvestigatorDetails,
    });
    app.insert_resource(RoleState {
        current: Role::Ghost,
    });
    app.insert_resource(crate::core::RoleYaw {
        ghost: 0.0,
        investigator: 0.0,
    });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(PuzzleSpawned(false));
    app.insert_resource(InvestigationState::default());
    app.insert_resource(SessionState { started: false });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostState {
        position: Vec3::new(0.0, 1.6, 0.0),
    });
    app.insert_resource(HouseLayout {
        bounds: Bounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
        },
        obstacles: Vec::new(),
        rooms: vec![
            RoomZone {
                id: 0,
                name: "Custom Main",
                bounds: Bounds {
                    min_x: -10.0,
                    max_x: 0.0,
                    min_z: -10.0,
                    max_z: 10.0,
                },
            },
            RoomZone {
                id: 1,
                name: "Custom Side",
                bounds: Bounds {
                    min_x: 0.0,
                    max_x: 10.0,
                    min_z: -10.0,
                    max_z: 10.0,
                },
            },
        ],
        walls: Vec::new(),
        exorcism: crate::gameplay::map::components::ExorcismLayout {
            spirit_anchors: vec![Vec3::new(-8.0, 0.7, -4.0)],
            banshee_anchors: vec![Vec3::new(8.0, 0.5, 0.0), Vec3::new(9.0, 0.5, 0.0)],
            onryo_cursed_positions: vec![Vec3::new(-8.0, 0.4, 0.0)],
            onryo_ritual_positions: vec![Vec3::new(8.0, 0.1, 3.0)],
        },
        investigator_spawn: Vec3::new(-8.0, 0.9, -3.0),
        ghost_spawns: vec![Vec3::new(8.0, 1.6, 3.0)],
    });

    let player = app
        .world_mut()
        .spawn((
            Transform::from_xyz(1.0, 0.9, 1.0),
            GlobalTransform::default(),
            crate::gameplay::investigator::Player,
        ))
        .id();

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        BackgroundColor(Color::BLACK),
        BeginInvestigationButton,
    ));

    app.update();

    let player_transform = app.world().entity(player).get::<Transform>().unwrap();
    assert_eq!(player_transform.translation, Vec3::new(-8.0, 0.9, -3.0));
    assert_eq!(
        app.world().resource::<GhostState>().position,
        Vec3::new(8.0, 1.6, 3.0)
    );
}

#[test]
fn escape_returns_to_role_select_from_details() {
    let mut app = App::new();
    app.add_systems(Update, crate::ui::lobby::handle_menu_toggle);
    app.insert_resource(MenuState {
        open: true,
        selected_role: Role::Ghost,
    });
    app.insert_resource(MenuFlowState {
        screen: MenuScreen::GhostDetails,
    });
    app.insert_resource(SessionState { started: false });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(ButtonInput::<KeyCode>::default());

    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::Escape);
    }
    app.update();

    let menu = app.world().resource::<MenuState>();
    let flow = app.world().resource::<MenuFlowState>();
    assert!(menu.open);
    assert!(matches!(flow.screen, MenuScreen::RoleSelect));
}

#[test]
fn cursor_unlocks_when_journal_open() {
    let mut app = App::new();
    app.add_systems(Update, crate::ui::lobby::update_cursor_lock);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(JournalState { open: true });

    let window_entity = app
        .world_mut()
        .spawn((
            Window {
                cursor: Cursor {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::Locked,
                    ..default()
                },
                ..default()
            },
            PrimaryWindow,
        ))
        .id();

    app.update();

    let window = app.world().entity(window_entity).get::<Window>().unwrap();
    assert_eq!(window.cursor.grab_mode, bevy::window::CursorGrabMode::None);
    assert!(window.cursor.visible);
}

#[test]
fn emf_updates_even_when_spiritbox_active() {
    let mut app = App::new();
    app.add_systems(Update, update_emf_reading);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostState {
        position: Vec3::new(0.0, 0.0, 0.0),
    });
    app.insert_resource(CameraControl { yaw: 0.0, pitch: 0.0 });
    app.insert_resource(Time::<()>::default());
    app.insert_resource(EquipmentState {
        active: Equipment::Spiritbox,
        emf_level: 0,
        emf_dwell: 0.0,
        emf_smoothed: 0.0,
        emf_evidence_latch: 0.0,
        spiritbox_message: "Silence...".to_string(),
        spiritbox_cooldown: 0.0,
    });
    app.insert_resource(EvidenceState::default());
    app.insert_resource(EvidenceTuning::default());

    app.world_mut().spawn((
        Transform::from_xyz(0.0, 0.0, 0.5),
        GlobalTransform::default(),
        crate::gameplay::investigator::Player,
    ));
    app.world_mut().spawn(Camera3dBundle::default());

    for _ in 0..5 {
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(std::time::Duration::from_secs_f32(0.2));
        }
        app.update();
    }

    let equipment = app.world().resource::<EquipmentState>();
    assert_eq!(equipment.emf_level, 5);
}

#[test]
fn spirit_markers_spawn_with_puzzle() {
    let mut app = App::new();
    app.add_plugins(ExorcismPlugin);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(GhostState {
        position: Vec3::ZERO,
    });
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.insert_resource(Time::<()>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());

    {
        let mut investigation = app.world_mut().resource_mut::<InvestigationState>();
        investigation.guess = Some(GhostType::Spirit);
        investigation.confirmed = true;
    }

    app.update();

    let world = app.world_mut();
    let mut query = world.query::<&SpiritMarker>();
    let count = query.iter(world).count();
    assert!(count > 0);
}

#[test]
fn spirit_markers_use_house_layout_anchor_positions() {
    let mut app = App::new();
    app.add_plugins(ExorcismPlugin);
    app.insert_resource(MenuState {
        open: false,
        selected_role: Role::Investigator,
    });
    app.insert_resource(RoleState {
        current: Role::Investigator,
    });
    app.insert_resource(JournalState { open: false });
    app.insert_resource(GhostTypeState {
        selected: GhostType::Spirit,
        active: GhostType::Spirit,
    });
    app.insert_resource(GhostState {
        position: Vec3::ZERO,
    });
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.insert_resource(Time::<()>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(HouseLayout {
        bounds: Bounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
        },
        obstacles: Vec::new(),
        rooms: vec![RoomZone {
            id: 0,
            name: "Only Room",
            bounds: Bounds {
                min_x: -10.0,
                max_x: 10.0,
                min_z: -10.0,
                max_z: 10.0,
            },
        }],
        walls: Vec::new(),
        exorcism: crate::gameplay::map::components::ExorcismLayout {
            spirit_anchors: vec![Vec3::new(-7.0, 0.7, -3.0), Vec3::new(7.0, 0.7, 3.0)],
            banshee_anchors: vec![Vec3::new(0.0, 0.5, 0.0)],
            onryo_cursed_positions: vec![Vec3::new(-3.0, 0.4, 0.0)],
            onryo_ritual_positions: vec![Vec3::new(3.0, 0.1, 0.0)],
        },
        investigator_spawn: Vec3::new(0.0, 0.9, 0.0),
        ghost_spawns: vec![Vec3::new(1.0, 1.6, 1.0)],
    });

    {
        let mut investigation = app.world_mut().resource_mut::<InvestigationState>();
        investigation.guess = Some(GhostType::Spirit);
        investigation.confirmed = true;
    }

    app.update();

    let world = app.world_mut();
    let mut query = world.query::<(&SpiritMarker, &Transform)>();
    let mut positions = query
        .iter(world)
        .map(|(_, transform)| Vec3::new(transform.translation.x, 0.7, transform.translation.z))
        .collect::<Vec<_>>();
    positions.sort_by(|a, b| a.x.total_cmp(&b.x));

    assert_eq!(positions.len(), 2);
    assert_eq!(positions[0], Vec3::new(-7.0, 0.7, -3.0));
    assert_eq!(positions[1], Vec3::new(7.0, 0.7, 3.0));
}
