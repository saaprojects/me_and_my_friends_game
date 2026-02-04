use crate::prelude::*;

use crate::core::{JournalState, MenuState, RoleState};
use crate::gameplay::investigator::tools::{EvidenceState, EquipmentState};
use crate::gameplay::exorcism::{ExorcismState, ExorcismStatus, InvestigationState};
use crate::gameplay::exorcism::tables::{puzzle_name, ExorcismTables};
use crate::ui::{
    EmfText, HudRoot, JournalEmfText, JournalGuessText, JournalSpiritText, PuzzleDetailText,
    PuzzleStatusText, PuzzleTitleText, SpiritboxText, ToolText, GhostHudRoot, GhostAbilityText,
    JournalSelectSpiritButton, JournalSelectBansheeButton, JournalSelectOnryoButton,
    JournalConfirmButton, JournalConfirmText, JournalSection,
};

pub fn setup_hud(mut commands: Commands) {
    let panel_color = BackgroundColor(Color::srgba(0.05, 0.08, 0.15, 0.75));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Px(24.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    row_gap: Val::Px(6.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: panel_color,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "TOOLS",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.65, 0.7, 1.0),
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Active: EMF Reader",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ToolText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "EMF: --",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.75, 0.8, 0.95),
                        ..default()
                    },
                ),
                EmfText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Spiritbox: --",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.8, 0.78, 0.9),
                        ..default()
                    },
                ),
                SpiritboxText,
            ));
            parent.spawn(TextBundle::from_section(
                "1 = EMF | 2 = Spiritbox | E = Ask | F = Interact",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.65, 0.7, 0.85),
                    ..default()
                },
            ));

            parent.spawn((
                TextBundle::from_section(
                    "JOURNAL",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.65, 0.7, 1.0),
                        ..default()
                    },
                ),
                JournalSection,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "EMF 5: No",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.78, 0.83, 0.95),
                        ..default()
                    },
                ),
                JournalEmfText,
                JournalSection,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Spiritbox Response: No",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.78, 0.83, 0.95),
                        ..default()
                    },
                ),
                JournalSpiritText,
                JournalSection,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Guess: Onryo",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.9, 1.0),
                        ..default()
                    },
                ),
                JournalGuessText,
                JournalSection,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "CONFIRM GHOST",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.65, 0.7, 1.0),
                        ..default()
                    },
                ),
                JournalSection,
            ));

            let button_style = Style {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                ..default()
            };
            let button_color = BackgroundColor(Color::srgba(0.2, 0.25, 0.4, 0.9));

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_color,
                        ..default()
                    },
                    JournalSelectSpiritButton,
                    JournalSection,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Spirit",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_color,
                        ..default()
                    },
                    JournalSelectBansheeButton,
                    JournalSection,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Banshee",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_color,
                        ..default()
                    },
                    JournalSelectOnryoButton,
                    JournalSection,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Onryo",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95)),
                        ..default()
                    },
                    JournalConfirmButton,
                    JournalSection,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Confirm",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent.spawn((
                TextBundle::from_section(
                    "Confirmed: -",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.75, 0.8, 0.95),
                        ..default()
                    },
                ),
                JournalConfirmText,
                JournalSection,
            ));

            parent.spawn(TextBundle::from_section(
                "PUZZLE",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.65, 0.7, 1.0),
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Spirit: The Vigil",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.9, 1.0),
                        ..default()
                    },
                ),
                PuzzleTitleText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Status: Inactive",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.78, 0.83, 0.95),
                        ..default()
                    },
                ),
                PuzzleStatusText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Progress: 0%",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.78, 0.83, 0.95),
                        ..default()
                    },
                ),
                PuzzleDetailText,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    right: Val::Px(24.0),
                    width: Val::Px(260.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    row_gap: Val::Px(6.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: panel_color,
                ..default()
            },
            GhostHudRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GHOST ABILITIES",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.65, 0.7, 1.0),
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "L: Toggle room lights",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.9, 1.0),
                        ..default()
                    },
                ),
                GhostAbilityText,
            ));
        });
}

pub fn sync_hud_visibility(
    menu: Res<MenuState>,
    role: Res<RoleState>,
    mut root: Query<&mut Visibility, With<HudRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open || role.current == Role::Ghost {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
}

pub fn sync_ghost_hud_visibility(
    menu: Res<MenuState>,
    role: Res<RoleState>,
    mut root: Query<&mut Visibility, With<GhostHudRoot>>,
) {
    let mut visibility = root.single_mut();
    *visibility = if menu.open || role.current != Role::Ghost {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
}

pub fn handle_journal_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    mut journal: ResMut<JournalState>,
) {
    if menu.open || role.current != Role::Investigator {
        return;
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        journal.open = !journal.open;
    }
}

pub fn sync_journal_panel_visibility(
    menu: Res<MenuState>,
    role: Res<RoleState>,
    journal: Res<JournalState>,
    mut items: Query<&mut Visibility, With<JournalSection>>,
) {
    if menu.open || role.current != Role::Investigator {
        for mut visibility in items.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        return;
    }

    let visibility = if journal.open {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut item_visibility in items.iter_mut() {
        *item_visibility = visibility;
    }
}

pub fn handle_journal_interactions(
    mut interactions: Query<
        (
            &Interaction,
            Option<&JournalSelectSpiritButton>,
            Option<&JournalSelectBansheeButton>,
            Option<&JournalSelectOnryoButton>,
            Option<&JournalConfirmButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    mut investigation: ResMut<InvestigationState>,
    mut puzzle_spawned: ResMut<crate::gameplay::exorcism::PuzzleSpawned>,
) {
    if menu.open || role.current != Role::Investigator || investigation.confirmed || !journal.open {
        return;
    }

    for (interaction, spirit_btn, banshee_btn, onryo_btn, confirm_btn) in interactions.iter_mut()
    {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if spirit_btn.is_some() {
            investigation.guess = Some(crate::core::GhostType::Spirit);
        } else if banshee_btn.is_some() {
            investigation.guess = Some(crate::core::GhostType::Banshee);
        } else if onryo_btn.is_some() {
            investigation.guess = Some(crate::core::GhostType::Onryo);
        } else if confirm_btn.is_some() {
            if investigation.guess.is_some() {
                investigation.confirmed = true;
                puzzle_spawned.0 = false;
            }
        }
    }
}

pub fn sync_journal_styles(
    investigation: Res<InvestigationState>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    mut buttons: Query<(
        &mut BackgroundColor,
        Option<&JournalSelectSpiritButton>,
        Option<&JournalSelectBansheeButton>,
        Option<&JournalSelectOnryoButton>,
        Option<&JournalConfirmButton>,
    )>,
    mut confirm_text: Query<&mut Text, With<JournalConfirmText>>,
) {
    if menu.open || role.current != Role::Investigator || !journal.open {
        return;
    }

    let selected_color = BackgroundColor(Color::srgba(0.2, 0.45, 0.95, 0.95));
    let idle_color = BackgroundColor(Color::srgba(0.2, 0.25, 0.4, 0.9));
    let confirm_color = BackgroundColor(Color::srgba(0.2, 0.55, 0.3, 0.95));

    for (mut color, spirit_btn, banshee_btn, onryo_btn, confirm_btn) in buttons.iter_mut() {
        if spirit_btn.is_some() {
            *color = if investigation.guess == Some(crate::core::GhostType::Spirit) {
                selected_color
            } else {
                idle_color
            };
        } else if banshee_btn.is_some() {
            *color = if investigation.guess == Some(crate::core::GhostType::Banshee) {
                selected_color
            } else {
                idle_color
            };
        } else if onryo_btn.is_some() {
            *color = if investigation.guess == Some(crate::core::GhostType::Onryo) {
                selected_color
            } else {
                idle_color
            };
        } else if confirm_btn.is_some() {
            *color = if investigation.confirmed {
                confirm_color
            } else {
                idle_color
            };
        }
    }

    if let Ok(mut text) = confirm_text.get_single_mut() {
        let label = if investigation.confirmed {
            match investigation.guess {
                Some(crate::core::GhostType::Spirit) => "Confirmed: Spirit",
                Some(crate::core::GhostType::Banshee) => "Confirmed: Banshee",
                Some(crate::core::GhostType::Onryo) => "Confirmed: Onryo",
                None => "Confirmed: -",
            }
        } else {
            "Confirmed: -"
        };
        text.sections[0].value = label.to_string();
    }
}

pub fn sync_journal_visibility(
    investigation: Res<InvestigationState>,
    role: Res<RoleState>,
    menu: Res<MenuState>,
    journal: Res<JournalState>,
    mut items: Query<(
        &mut Visibility,
        Option<&JournalSelectSpiritButton>,
        Option<&JournalSelectBansheeButton>,
        Option<&JournalSelectOnryoButton>,
        Option<&JournalConfirmButton>,
    )>,
) {
    if menu.open || role.current != Role::Investigator || !journal.open {
        return;
    }

    let show = !investigation.confirmed;
    for (mut visibility, spirit_btn, banshee_btn, onryo_btn, confirm_btn) in items.iter_mut() {
        if spirit_btn.is_some()
            || banshee_btn.is_some()
            || onryo_btn.is_some()
            || confirm_btn.is_some()
        {
            *visibility = if show {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn sync_hud_text(
    role: Res<RoleState>,
    equipment: Res<EquipmentState>,
    evidence: Res<EvidenceState>,
    ghost_type: Res<crate::core::GhostTypeState>,
    exorcism: Res<ExorcismStatus>,
    investigation: Res<InvestigationState>,
    tables: Res<ExorcismTables>,
    mut texts: Query<(
        &mut Text,
        Option<&ToolText>,
        Option<&EmfText>,
        Option<&SpiritboxText>,
        Option<&JournalEmfText>,
        Option<&JournalSpiritText>,
        Option<&JournalGuessText>,
        Option<&PuzzleTitleText>,
        Option<&PuzzleStatusText>,
        Option<&PuzzleDetailText>,
    )>,
) {
    let active_label = match equipment.active {
        crate::core::Equipment::Emf => "EMF Reader",
        crate::core::Equipment::Spiritbox => "Spiritbox",
    };

    let emf_value = if role.current == Role::Investigator
        && equipment.active == crate::core::Equipment::Emf
    {
        if equipment.emf_level == 0 {
            "0".to_string()
        } else {
            equipment.emf_level.to_string()
        }
    } else {
        "--".to_string()
    };

    let guess = if evidence.emf_five {
        "Spirit"
    } else if evidence.spiritbox_response {
        "Banshee"
    } else {
        "Onryo"
    };

    let puzzle_label = puzzle_name(investigation.guess.unwrap_or(ghost_type.active));
    let sequence_len = tables.banshee.sequence_len();

    let awaiting = !investigation.confirmed || investigation.guess.is_none();
    for (
        mut text,
        tool_tag,
        emf_tag,
        spirit_tag,
        journal_emf,
        journal_spirit,
        journal_guess,
        puzzle_title,
        puzzle_status_tag,
        puzzle_detail,
    ) in texts.iter_mut()
    {
        if tool_tag.is_some() {
            text.sections[0].value = format!("Active: {}", active_label);
        } else if emf_tag.is_some() {
            text.sections[0].value = format!("EMF: {}", emf_value);
        } else if spirit_tag.is_some() {
            text.sections[0].value = format!("Spiritbox: {}", equipment.spiritbox_message);
        } else if journal_emf.is_some() {
            text.sections[0].value = format!(
                "EMF 5: {}",
                if evidence.emf_five { "Yes" } else { "No" }
            );
        } else if journal_spirit.is_some() {
            text.sections[0].value = format!(
                "Spiritbox Response: {}",
                if evidence.spiritbox_response { "Yes" } else { "No" }
            );
        } else if journal_guess.is_some() {
            text.sections[0].value = format!("Guess: {}", guess);
        } else if puzzle_title.is_some() {
            text.sections[0].value = if awaiting {
                "Puzzle: Awaiting Confirmation".to_string()
            } else {
                puzzle_label.to_string()
            };
        } else if puzzle_status_tag.is_some() {
            let status_text = if awaiting {
                "Status: Waiting on confirmation".to_string()
            } else {
                match exorcism.state {
                    ExorcismState::Stage(stage) => format!("Status: Stage {}", stage + 1),
                    ExorcismState::Progress(progress) => {
                        format!("Status: {}%", (progress * 100.0).round() as u32)
                    }
                    ExorcismState::Complete => "Status: Complete".to_string(),
                    ExorcismState::Failed => "Status: Failed".to_string(),
                    ExorcismState::Inactive => "Status: Inactive".to_string(),
                }
            };
            text.sections[0].value = status_text;
        } else if puzzle_detail.is_some() {
            let detail = if awaiting {
                "Progress: -".to_string()
            } else {
                match investigation.guess.unwrap_or(ghost_type.active) {
                    crate::core::GhostType::Spirit => {
                        format!("Progress: {}%", (exorcism.progress * 100.0).round() as u32)
                    }
                    crate::core::GhostType::Banshee => {
                        if matches!(exorcism.state, ExorcismState::Complete) {
                            "Sequence: Complete".to_string()
                        } else {
                            let step = exorcism.stage.min(sequence_len.saturating_sub(1)) + 1;
                            format!("Sequence: {}/{}", step, sequence_len.max(1))
                        }
                    }
                    crate::core::GhostType::Onryo => {
                        format!(
                            "Stacks: {}/{}",
                            exorcism.stacks.ceil() as u32,
                            exorcism.max_stacks as u32
                        )
                    }
                }
            };
            text.sections[0].value = detail;
        }
    }
}
