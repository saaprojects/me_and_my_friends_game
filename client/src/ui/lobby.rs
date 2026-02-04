use crate::prelude::*;

use crate::core::{
    GhostTypeState, JournalState, MenuFlowState, MenuScreen, MenuState, Role, RoleState, RoleYaw,
    SessionState,
};
use crate::gameplay::exorcism::{InvestigationState, PuzzleSpawned};
use crate::gameplay::investigator::tools::EvidenceState;
use crate::ui::{
    BeginHauntButton, BeginInvestigationButton, ExitButton, GhostDetailRoot, GhostSelectButton,
    InvestigatorDetailRoot, InvestigatorSelectButton, OnryoGhostButton, RoleSelectRoot,
    SpiritGhostButton, StartScreenButton, StartScreenRoot,
    BansheeGhostButton,
};

pub fn setup_menu(mut commands: Commands) {
    let background = BackgroundColor(Color::srgb(0.04, 0.06, 0.1));
    let panel = BackgroundColor(Color::srgba(0.06, 0.08, 0.13, 0.9));
    let ghost_panel = BackgroundColor(Color::srgba(0.08, 0.1, 0.16, 0.92));
    let investigator_panel = BackgroundColor(Color::srgba(0.06, 0.09, 0.14, 0.92));
    let button_color = BackgroundColor(Color::srgba(0.2, 0.25, 0.4, 0.9));
    let primary_button = BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(24.0),
                    ..default()
                },
                background_color: background,
                ..default()
            },
            StartScreenRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ME AND MY FRIENDS",
                TextStyle {
                    font_size: 48.0,
                    color: Color::srgb(0.85, 0.9, 1.0),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Asymmetric Ghost Hunt",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.6, 0.7, 0.85),
                    ..default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(24.0), Val::Px(14.0)),
                            ..default()
                        },
                        background_color: primary_button,
                        ..default()
                    },
                    StartScreenButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(24.0),
                    padding: UiRect::top(Val::Px(28.0)),
                    ..default()
                },
                background_color: background,
                ..default()
            },
            RoleSelectRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ME AND MY FRIENDS",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.8, 0.86, 1.0),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(92.0),
                        height: Val::Percent(70.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(18.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                })
                .with_children(|row| {
                    row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(24.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(12.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: ghost_panel,
                                ..default()
                            },
                            GhostSelectButton,
                        ))
                        .with_children(|column| {
                            column.spawn(TextBundle::from_section(
                                "Ghost",
                                TextStyle {
                                    font_size: 28.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                            column.spawn(TextBundle::from_section(
                                "Haunt the house and mislead the investigators.",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::srgb(0.7, 0.75, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(24.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(12.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: investigator_panel,
                                ..default()
                            },
                            InvestigatorSelectButton,
                        ))
                        .with_children(|column| {
                            column.spawn(TextBundle::from_section(
                                "Investigator",
                                TextStyle {
                                    font_size: 28.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                            column.spawn(TextBundle::from_section(
                                "Gather evidence and exorcise the ghost.",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::srgb(0.7, 0.75, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.6, 0.2, 0.2, 0.95)),
                        ..default()
                    },
                    ExitButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(18.0),
                    padding: UiRect::top(Val::Px(28.0)),
                    ..default()
                },
                background_color: panel,
                ..default()
            },
            GhostDetailRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Ghost Loadout",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.85, 0.9, 1.0),
                    ..default()
                },
            ));

            let ghost_button_style = Style {
                padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
                width: Val::Px(260.0),
                ..default()
            };

            parent
                .spawn((
                    ButtonBundle {
                        style: ghost_button_style.clone(),
                        background_color: button_color,
                        ..default()
                    },
                    SpiritGhostButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Spirit (EMF 5)",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: ghost_button_style.clone(),
                        background_color: button_color,
                        ..default()
                    },
                    BansheeGhostButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Banshee (Spiritbox)",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: ghost_button_style,
                        background_color: button_color,
                        ..default()
                    },
                    OnryoGhostButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Onryo (No Evidence)",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: primary_button,
                        ..default()
                    },
                    BeginHauntButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Begin Haunt",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(18.0),
                    padding: UiRect::top(Val::Px(28.0)),
                    ..default()
                },
                background_color: panel,
                ..default()
            },
            InvestigatorDetailRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Investigator",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.85, 0.9, 1.0),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Cosmetics and loadouts coming soon.",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.7, 0.75, 0.9),
                    ..default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: primary_button,
                        ..default()
                    },
                    BeginInvestigationButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Begin Investigation",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

pub fn handle_menu_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuState>,
    mut journal: ResMut<JournalState>,
    mut flow: ResMut<MenuFlowState>,
    session: Res<SessionState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if menu.open
            && matches!(
                flow.screen,
                MenuScreen::GhostDetails | MenuScreen::InvestigatorDetails
            )
        {
            flow.screen = MenuScreen::RoleSelect;
            return;
        }
        menu.open = !menu.open;
        if menu.open {
            journal.open = false;
            if session.started {
                flow.screen = MenuScreen::RoleSelect;
            }
        }
    }
}

pub fn handle_menu_interactions(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartScreenButton>,
            Option<&GhostSelectButton>,
            Option<&InvestigatorSelectButton>,
            Option<&SpiritGhostButton>,
            Option<&BansheeGhostButton>,
            Option<&OnryoGhostButton>,
            Option<&BeginHauntButton>,
            Option<&BeginInvestigationButton>,
            Option<&ExitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: ResMut<MenuState>,
    mut flow: ResMut<MenuFlowState>,
    mut role: ResMut<RoleState>,
    mut role_yaw: ResMut<RoleYaw>,
    mut ghost_type: ResMut<GhostTypeState>,
    mut evidence: ResMut<EvidenceState>,
    mut puzzle_spawned: ResMut<PuzzleSpawned>,
    mut investigation: ResMut<InvestigationState>,
    mut session: ResMut<SessionState>,
    mut control: ResMut<crate::core::CameraControl>,
    mut journal: ResMut<JournalState>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (
        interaction,
        mut color,
        start_screen_btn,
        ghost_btn,
        investigator_btn,
        spirit_btn,
        banshee_btn,
        onryo_btn,
        begin_haunt_btn,
        begin_investigation_btn,
        exit_btn,
    ) in interactions.iter_mut()
    {
        if let Interaction::Pressed = interaction {
            if start_screen_btn.is_some() {
                flow.screen = MenuScreen::RoleSelect;
            }
            if ghost_btn.is_some() {
                menu.selected_role = Role::Ghost;
                flow.screen = MenuScreen::GhostDetails;
            }
            if investigator_btn.is_some() {
                menu.selected_role = Role::Investigator;
                flow.screen = MenuScreen::InvestigatorDetails;
            }
            if spirit_btn.is_some() {
                ghost_type.selected = GhostType::Spirit;
            }
            if banshee_btn.is_some() {
                ghost_type.selected = GhostType::Banshee;
            }
            if onryo_btn.is_some() {
                ghost_type.selected = GhostType::Onryo;
            }

            let begin_haunt = begin_haunt_btn.is_some();
            let begin_investigation = begin_investigation_btn.is_some();
            if begin_haunt || begin_investigation {
                if begin_haunt {
                    role.current = Role::Ghost;
                    menu.selected_role = Role::Ghost;
                } else {
                    role.current = Role::Investigator;
                    menu.selected_role = Role::Investigator;
                }
                ghost_type.active = ghost_type.selected;
                *evidence = EvidenceState::default();
                puzzle_spawned.0 = false;
                investigation.guess = None;
                investigation.confirmed = false;
                session.started = true;
                set_default_camera(role.current, &mut control, &mut role_yaw);
                menu.open = false;
                journal.open = false;
                flow.screen = MenuScreen::RoleSelect;
            }

            if exit_btn.is_some() {
                exit_events.send(AppExit::Success);
            }

            *color = BackgroundColor(Color::srgba(0.3, 0.35, 0.55, 0.95));
        }
    }
}

pub fn sync_start_screen_visibility(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut root: Query<&mut Visibility, With<StartScreenRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open && flow.screen == MenuScreen::Start {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn sync_role_select_visibility(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut root: Query<&mut Visibility, With<RoleSelectRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open && flow.screen == MenuScreen::RoleSelect {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn sync_ghost_detail_visibility(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut root: Query<&mut Visibility, With<GhostDetailRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open && flow.screen == MenuScreen::GhostDetails {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn sync_investigator_detail_visibility(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut root: Query<&mut Visibility, With<InvestigatorDetailRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open && flow.screen == MenuScreen::InvestigatorDetails {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn sync_menu_styles(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    ghost_type: Res<GhostTypeState>,
    mut buttons: Query<(
        &mut BackgroundColor,
        Option<&GhostSelectButton>,
        Option<&InvestigatorSelectButton>,
        Option<&SpiritGhostButton>,
        Option<&BansheeGhostButton>,
        Option<&OnryoGhostButton>,
    )>,
) {
    let selected_color = BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95));
    let idle_color = BackgroundColor(Color::srgba(0.2, 0.25, 0.4, 0.9));

    for (mut color, ghost_btn, investigator_btn, spirit_btn, banshee_btn, onryo_btn) in
        buttons.iter_mut()
    {
        if ghost_btn.is_some() {
            if flow.screen == MenuScreen::RoleSelect {
                continue;
            }
            *color = if menu.selected_role == Role::Ghost {
                selected_color
            } else {
                idle_color
            };
        } else if investigator_btn.is_some() {
            if flow.screen == MenuScreen::RoleSelect {
                continue;
            }
            *color = if menu.selected_role == Role::Investigator {
                selected_color
            } else {
                idle_color
            };
        } else if spirit_btn.is_some() {
            *color = if ghost_type.selected == GhostType::Spirit {
                selected_color
            } else {
                idle_color
            };
        } else if banshee_btn.is_some() {
            *color = if ghost_type.selected == GhostType::Banshee {
                selected_color
            } else {
                idle_color
            };
        } else if onryo_btn.is_some() {
            *color = if ghost_type.selected == GhostType::Onryo {
                selected_color
            } else {
                idle_color
            };
        }
    }
}

pub fn sync_role_select_hover(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut buttons: Query<(
        &Interaction,
        &mut BackgroundColor,
        Option<&GhostSelectButton>,
        Option<&InvestigatorSelectButton>,
    )>,
) {
    if !menu.open || flow.screen != MenuScreen::RoleSelect {
        return;
    }

    let hover_color = BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95));
    let idle_color = BackgroundColor(Color::srgba(0.08, 0.1, 0.16, 0.92));

    for (interaction, mut color, ghost_btn, investigator_btn) in buttons.iter_mut() {
        if ghost_btn.is_some() || investigator_btn.is_some() {
            *color = if *interaction == Interaction::Hovered {
                hover_color
            } else {
                idle_color
            };
        }
    }
}

pub fn update_cursor_lock(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
) {
    let mut window = windows.single_mut();
    if menu.open || journal.open {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    } else {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}
