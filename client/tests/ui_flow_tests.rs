use bevy::prelude::*;
use bevy::window::{Cursor, PrimaryWindow};

use client::core::{
    Equipment, GhostType, GhostTypeState, JournalState, MenuFlowState, MenuScreen, MenuState, Role,
    RoleState, SessionState,
};
use client::core::CameraControl;
use client::gameplay::evidence::EvidenceTuning;
use client::gameplay::exorcism::{
    ExorcismPlugin, ExorcismStatus, InvestigationState, PuzzleSpawned, SpiritMarker,
};
use client::gameplay::ghost::GhostState;
use client::gameplay::investigator::tools::{update_emf_reading, EquipmentState, EvidenceState};
use client::ui::hud;
use client::ui::*;

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
        state: client::gameplay::exorcism::ExorcismState::Inactive,
        progress: 0.0,
        stage: 0,
        stacks: 0.0,
        max_stacks: 0.0,
    });
    app.insert_resource(client::gameplay::exorcism::tables::ExorcismTables::default());
    app.insert_resource(EquipmentState {
        active: Equipment::Emf,
        emf_level: 0,
        emf_dwell: 0.0,
        emf_smoothed: 0.0,
        emf_evidence_latch: 0.0,
        spiritbox_message: "Silence...".to_string(),
        spiritbox_cooldown: 0.0,
    });
    app.insert_resource(client::gameplay::investigator::tools::EvidenceState::default());
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
    app.add_systems(Update, client::ui::lobby::sync_start_screen_visibility);
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
    app.add_systems(Update, client::ui::lobby::sync_role_select_visibility);
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
    app.add_systems(Update, client::ui::lobby::handle_menu_interactions);
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
    app.insert_resource(client::core::RoleYaw {
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
fn escape_returns_to_role_select_from_details() {
    let mut app = App::new();
    app.add_systems(Update, client::ui::lobby::handle_menu_toggle);
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
    app.add_systems(Update, client::ui::lobby::update_cursor_lock);
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
        client::gameplay::investigator::Player,
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
