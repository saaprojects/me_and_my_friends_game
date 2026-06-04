use crate::prelude::*;

use crate::core::{
    GhostTypeState, JournalState, MenuFlowState, MenuScreen, MenuState, ResolutionState, Role,
    RoleState, RoleYaw, RoundOutcome, SessionState,
};
use crate::core::GameRng;
use crate::gameplay::exorcism::{ExorcismState, ExorcismStatus, InvestigationState, PuzzleSpawned, RoomLights};
use crate::gameplay::ghost::GhostState;
use crate::gameplay::investigator::tools::{EquipmentState, EvidenceState};
use crate::gameplay::investigator::Player;
use crate::gameplay::map::components::CollisionWorld;
use crate::gameplay::map::systems::random_round_start_positions;
use crate::gameplay::map::{HouseLayout, HouseLayoutKind, HouseLayoutSelection};
use crate::ui::{
    ButtonAnimT, ButtonKind, GhostDetailRoot, InvestigatorDetailRoot, ResolutionBodyText,
    ResolutionRoot, ResolutionTitleText, RoleSelectRoot, StartScreenRoot,
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
                    ButtonKind::StartScreen,
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
                    row.spawn((
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
                        ButtonKind::GhostSelect,
                        ButtonAnimT::default(),
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

                    row.spawn((
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
                        ButtonKind::InvestigatorSelect,
                        ButtonAnimT::default(),
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
                    ButtonKind::Exit,
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
                    ButtonKind::SpiritGhost,
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
                    ButtonKind::BansheeGhost,
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
                    ButtonKind::OnryoGhost,
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

            parent.spawn(TextBundle::from_section(
                "Room Count",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.86, 1.0),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                                ..default()
                            },
                            background_color: button_color,
                            ..default()
                        },
                        ButtonKind::TwoRoomCount,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "2 Rooms",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });

                    row.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                                ..default()
                            },
                            background_color: button_color,
                            ..default()
                        },
                        ButtonKind::ThreeRoomCount,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "3 Rooms",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
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
                    ButtonKind::BeginHaunt,
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
                    ButtonKind::BeginInvestigation,
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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(18.0),
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(24.0)),
                    ..default()
                },
                background_color: panel,
                ..default()
            },
            ResolutionRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Case Closed",
                    TextStyle {
                        font_size: 36.0,
                        color: Color::srgb(0.88, 0.92, 1.0),
                        ..default()
                    },
                ),
                ResolutionTitleText,
            ));
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Return to the role select when you're ready.",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.72, 0.78, 0.92),
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    style: Style {
                        max_width: Val::Px(640.0),
                        ..default()
                    },
                    ..default()
                },
                ResolutionBodyText,
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
                    ButtonKind::ResolutionContinue,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Back To Roles",
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
    input: Res<crate::core::InputMap>,
) {
    if keys.just_pressed(input.menu_toggle) {
        if menu.open
            && matches!(
                flow.screen,
                MenuScreen::GhostDetails | MenuScreen::InvestigatorDetails | MenuScreen::Resolution
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
        (&Interaction, &mut BackgroundColor, &ButtonKind),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: ResMut<MenuState>,
    mut flow: ResMut<MenuFlowState>,
    mut role: ResMut<RoleState>,
    mut role_yaw: ResMut<RoleYaw>,
    mut ghost_type: ResMut<GhostTypeState>,
    round_resources: (
        ResMut<EvidenceState>,
        ResMut<EquipmentState>,
        ResMut<PuzzleSpawned>,
        ResMut<InvestigationState>,
        ResMut<ResolutionState>,
        ResMut<SessionState>,
        ResMut<RoomLights>,
        ResMut<GameRng>,
    ),
    mut control: ResMut<crate::core::CameraControl>,
    mut journal: ResMut<JournalState>,
    mut ghost: Option<ResMut<GhostState>>,
    layout_resources: (
        Option<ResMut<HouseLayout>>,
        Option<ResMut<CollisionWorld>>,
        Option<ResMut<HouseLayoutSelection>>,
    ),
    mut players: Query<&mut Transform, With<Player>>,
    mut exit_events: EventWriter<AppExit>,
) {
    let (mut evidence, mut equipment, mut puzzle_spawned, mut investigation, mut resolution, mut session, mut lights, mut rng) =
        round_resources;
    let (mut active_house_layout, mut collision_world, mut house_selection) = layout_resources;

    for (interaction, mut color, kind) in interactions.iter_mut() {
        if let Interaction::Pressed = interaction {
            match kind {
                ButtonKind::StartScreen => { flow.screen = MenuScreen::RoleSelect; }
                ButtonKind::GhostSelect => {
                    menu.selected_role = Role::Ghost;
                    flow.screen = MenuScreen::GhostDetails;
                }
                ButtonKind::InvestigatorSelect => {
                    menu.selected_role = Role::Investigator;
                    flow.screen = MenuScreen::InvestigatorDetails;
                }
                ButtonKind::SpiritGhost => { ghost_type.selected = GhostType::Spirit; }
                ButtonKind::BansheeGhost => { ghost_type.selected = GhostType::Banshee; }
                ButtonKind::OnryoGhost => { ghost_type.selected = GhostType::Onryo; }
                ButtonKind::TwoRoomCount => {
                    if flow.screen == MenuScreen::GhostDetails {
                        if let Some(ref mut selection) = house_selection {
                            selection.selected_kind = HouseLayoutKind::TwoRoom;
                        }
                    }
                }
                ButtonKind::ThreeRoomCount => {
                    if flow.screen == MenuScreen::GhostDetails {
                        if let Some(ref mut selection) = house_selection {
                            selection.selected_kind = HouseLayoutKind::ThreeRoom;
                        }
                    }
                }
                _ => {}
            }

            let begin_haunt = *kind == ButtonKind::BeginHaunt;
            let begin_investigation = *kind == ButtonKind::BeginInvestigation;
            if begin_haunt || begin_investigation {
                let mut fresh_start_positions: Option<(Vec3, Vec3)> = None;
                if let Some(ref mut selection) = house_selection {
                    selection.active_kind = selection.selected_kind;
                    let new_layout = HouseLayout::for_kind(selection.active_kind);
                    fresh_start_positions = Some(new_layout.random_start_positions(&mut rng));
                    if let Some(ref mut collision) = collision_world {
                        **collision = new_layout.collision_world();
                    }
                    if let Some(ref mut layout) = active_house_layout {
                        **layout = new_layout;
                    }
                }

                if begin_haunt {
                    role.current = Role::Ghost;
                    menu.selected_role = Role::Ghost;
                } else {
                    role.current = Role::Investigator;
                    menu.selected_role = Role::Investigator;
                }
                ghost_type.active = ghost_type.selected;
                *evidence = EvidenceState::default();
                *equipment = EquipmentState::default();
                puzzle_spawned.0 = false;
                investigation.guess = None;
                investigation.confirmed = false;
                *resolution = ResolutionState::default();
                lights.reset(active_house_layout.as_deref());
                session.started = true;
                set_default_camera(role.current, &mut control, &mut role_yaw);
                let (investigator_spawn, ghost_spawn) = fresh_start_positions
                    .or_else(|| {
                        active_house_layout
                            .as_ref()
                            .map(|layout| layout.random_start_positions(&mut rng))
                    })
                    .unwrap_or_else(random_round_start_positions);
                if let Ok(mut player_transform) = players.get_single_mut() {
                    player_transform.translation = investigator_spawn;
                    player_transform.rotation = Quat::IDENTITY;
                }
                if let Some(ref mut ghost_state) = ghost {
                    ghost_state.position = ghost_spawn;
                }
                menu.open = false;
                journal.open = false;
                flow.screen = MenuScreen::RoleSelect;
            }

            if *kind == ButtonKind::Exit {
                exit_events.send(AppExit::Success);
            }

            *color = BackgroundColor(Color::srgba(0.3, 0.35, 0.55, 0.95));
        }
    }
}

pub fn handle_resolution_interactions(
    mut interactions: Query<
        (&Interaction, &ButtonKind),
        (Changed<Interaction>, With<Button>),
    >,
    mut flow: ResMut<MenuFlowState>,
) {
    for (interaction, kind) in interactions.iter_mut() {
        if *kind == ButtonKind::ResolutionContinue && *interaction == Interaction::Pressed {
            flow.screen = MenuScreen::RoleSelect;
        }
    }
}

pub fn maybe_open_resolution_screen(
    session: Res<SessionState>,
    exorcism: Res<ExorcismStatus>,
    investigation: Res<InvestigationState>,
    ghost_type: Res<GhostTypeState>,
    mut resolution: ResMut<ResolutionState>,
    mut menu: ResMut<MenuState>,
    mut journal: ResMut<JournalState>,
    mut flow: ResMut<MenuFlowState>,
) {
    if !session.started || resolution.shown || !investigation.confirmed {
        return;
    }

    let outcome = match exorcism.state {
        ExorcismState::Complete => Some(RoundOutcome::SuccessfulExorcism),
        ExorcismState::Failed => Some(if investigation.guess == Some(ghost_type.active) {
            RoundOutcome::FailedExorcism
        } else {
            RoundOutcome::WrongGhost
        }),
        _ => None,
    };

    let Some(outcome) = outcome else {
        return;
    };

    resolution.outcome = Some(outcome);
    resolution.shown = true;
    menu.open = true;
    journal.open = false;
    flow.screen = MenuScreen::Resolution;
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

pub fn sync_resolution_visibility(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut root: Query<&mut Visibility, With<ResolutionRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open && flow.screen == MenuScreen::Resolution {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub fn sync_resolution_text(
    resolution: Res<ResolutionState>,
    investigation: Res<InvestigationState>,
    ghost_type: Res<GhostTypeState>,
    mut texts: Query<(
        &mut Text,
        Option<&ResolutionTitleText>,
        Option<&ResolutionBodyText>,
    )>,
) {
    let guess_name = investigation
        .guess
        .map(ghost_type_name)
        .unwrap_or("Unknown");
    let actual_name = ghost_type_name(ghost_type.active);

    for (mut text, title_tag, body_tag) in texts.iter_mut() {
        if title_tag.is_some() {
            text.sections[0].value = match resolution.outcome {
                Some(RoundOutcome::SuccessfulExorcism) => "Ghost Banished".to_string(),
                Some(RoundOutcome::WrongGhost) => "Wrong Ghost".to_string(),
                Some(RoundOutcome::FailedExorcism) => "Exorcism Failed".to_string(),
                None => "Case Closed".to_string(),
            };
        } else if body_tag.is_some() {
            text.sections[0].value = match resolution.outcome {
                Some(RoundOutcome::SuccessfulExorcism) => format!(
                    "You identified the {} and finished the ritual. Head back to role select when you're ready for the next case.",
                    actual_name
                ),
                Some(RoundOutcome::WrongGhost) => format!(
                    "You confirmed {}, but the ghost was {}. Gather more evidence before locking in the next ritual.",
                    guess_name, actual_name
                ),
                Some(RoundOutcome::FailedExorcism) => format!(
                    "You had the right ghost, but the {} ritual broke down before completion. Reopen the case and try it again.",
                    actual_name
                ),
                None => "Return to the role select when you're ready.".to_string(),
            };
        }
    }
}

pub fn sync_menu_styles(
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    ghost_type: Res<GhostTypeState>,
    house_selection: Option<Res<HouseLayoutSelection>>,
    mut buttons: Query<(&mut BackgroundColor, &ButtonKind)>,
) {
    let selected_color = BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95));
    let idle_color = BackgroundColor(Color::srgba(0.2, 0.25, 0.4, 0.9));

    let selected_rooms = house_selection
        .as_ref()
        .map(|selection| selection.selected_kind)
        .unwrap_or(HouseLayoutKind::TwoRoom);

    for (mut color, kind) in buttons.iter_mut() {
        match kind {
            ButtonKind::GhostSelect => {
                if flow.screen == MenuScreen::RoleSelect { continue; }
                *color = if menu.selected_role == Role::Ghost { selected_color } else { idle_color };
            }
            ButtonKind::InvestigatorSelect => {
                if flow.screen == MenuScreen::RoleSelect { continue; }
                *color = if menu.selected_role == Role::Investigator { selected_color } else { idle_color };
            }
            ButtonKind::SpiritGhost => {
                *color = if ghost_type.selected == GhostType::Spirit { selected_color } else { idle_color };
            }
            ButtonKind::BansheeGhost => {
                *color = if ghost_type.selected == GhostType::Banshee { selected_color } else { idle_color };
            }
            ButtonKind::OnryoGhost => {
                *color = if ghost_type.selected == GhostType::Onryo { selected_color } else { idle_color };
            }
            ButtonKind::TwoRoomCount => {
                *color = if selected_rooms == HouseLayoutKind::TwoRoom { selected_color } else { idle_color };
            }
            ButtonKind::ThreeRoomCount => {
                *color = if selected_rooms == HouseLayoutKind::ThreeRoom { selected_color } else { idle_color };
            }
            _ => {}
        }
    }
}

pub fn sync_role_select_hover(
    time: Res<Time>,
    menu: Res<MenuState>,
    flow: Res<MenuFlowState>,
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &ButtonKind, &mut ButtonAnimT)>,
) {
    if !menu.open || flow.screen != MenuScreen::RoleSelect {
        return;
    }

    let delta = time.delta_seconds();
    for (interaction, mut color, kind, mut anim) in buttons.iter_mut() {
        if matches!(kind, ButtonKind::GhostSelect | ButtonKind::InvestigatorSelect) {
            let target = if *interaction == Interaction::Hovered { 1.0_f32 } else { 0.0 };
            anim.0 += (target - anim.0) * (delta * 10.0).min(1.0);
            let t = anim.0;
            *color = BackgroundColor(Color::srgba(
                0.08 + t * 0.12,
                0.10 + t * 0.35,
                0.16 + t * 0.79,
                0.92 + t * 0.03,
            ));
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

fn ghost_type_name(ghost_type: GhostType) -> &'static str {
    match ghost_type {
        GhostType::Spirit => "Spirit",
        GhostType::Banshee => "Banshee",
        GhostType::Onryo => "Onryo",
    }
}
