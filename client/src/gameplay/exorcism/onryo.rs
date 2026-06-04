use crate::prelude::*;

use crate::core::{GhostType, GhostTypeState, InputMap, JournalState, MenuState, Role, RoleState};
use crate::gameplay::exorcism::tables::ExorcismTables;
use crate::gameplay::exorcism::{ExorcismState, ExorcismStatus};
use crate::gameplay::map::ExorcismPlacement;
use super::rules;

#[derive(Component)]
pub struct OnryoCursed {
    pub placed: bool,
}

#[derive(Component)]
pub struct OnryoRitual {
    pub index: u8,
}

#[derive(Resource)]
pub struct OnryoPuzzle {
    pub stage: u8,
    pub stacks: f32,
    pub max_stacks: f32,
    pub carrying: Option<Entity>,
}

impl OnryoPuzzle {
    pub fn new(max_stacks: f32) -> Self {
        Self {
            stage: 0,
            stacks: 0.0,
            max_stacks,
            carrying: None,
        }
    }
}

pub fn update_onryo_puzzle(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<super::InvestigationState>,
    tables: Res<ExorcismTables>,
    journal: Res<JournalState>,
    input: Res<InputMap>,
    placement: Option<Res<ExorcismPlacement>>,
    house_layout: Option<Res<crate::gameplay::map::HouseLayout>>,
    player: Query<
        &Transform,
        (
            With<crate::gameplay::investigator::Player>,
            Without<OnryoCursed>,
            Without<OnryoRitual>,
        ),
    >,
    mut cursed: Query<
        (Entity, &mut Transform, &mut OnryoCursed),
        (
            Without<crate::gameplay::investigator::Player>,
            Without<OnryoRitual>,
        ),
    >,
    rituals: Query<
        (&OnryoRitual, &Transform),
        (
            Without<crate::gameplay::investigator::Player>,
            Without<OnryoCursed>,
        ),
    >,
    mut onryo: ResMut<OnryoPuzzle>,
    mut status: ResMut<ExorcismStatus>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    let Some(puzzle_type) = investigation.guess else {
        return;
    };
    if !investigation.confirmed || puzzle_type != GhostType::Onryo {
        return;
    }

    let max_stacks = tables.onryo.max_stacks;
    let (new_stacks, failed) = rules::onryo_stack_tick(
        onryo.stacks,
        time.delta_seconds(),
        onryo.carrying.is_some(),
        max_stacks,
        tables.onryo.stack_rate,
    );
    onryo.stacks = new_stacks;
    onryo.max_stacks = max_stacks;
    if failed {
        status.state = ExorcismState::Failed;
        return;
    }

    status.state = ExorcismState::Stage(onryo.stage);
    status.stage = onryo.stage;
    status.stacks = onryo.stacks;
    status.max_stacks = onryo.max_stacks;

    if !keys.just_pressed(input.interact) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    if let Some(carried_entity) = onryo.carrying {
        if let Ok((_, mut transform, cursed_obj)) = cursed.get_mut(carried_entity) {
            if !cursed_obj.placed {
                transform.translation =
                    player_transform.translation + Vec3::new(0.0, tables.onryo.carry_height, 0.0);
            }
        }
    }

    if onryo.carrying.is_none() {
        for (entity, transform, cursed_obj) in cursed.iter_mut() {
            if cursed_obj.placed {
                continue;
            }
            let distance = player_transform.translation.distance(transform.translation);
            if distance <= tables.onryo.interact_distance {
                onryo.carrying = Some(entity);
                onryo.stacks += tables.onryo.stack_penalty_wrong;
                break;
            }
        }
        return;
    }

    let mut target_spot: Option<(u8, Vec3)> = None;
    for (ritual, transform) in rituals.iter() {
        let distance = player_transform.translation.distance(transform.translation);
        if distance <= tables.onryo.interact_distance {
            target_spot = Some((ritual.index, transform.translation));
            break;
        }
    }

    let Some((spot_index, spot_pos)) = target_spot else {
        return;
    };

    let Some(carried_entity) = onryo.carrying.take() else {
        return;
    };

    if spot_index == onryo.stage {
        if let Ok((_, mut transform, mut cursed_obj)) = cursed.get_mut(carried_entity) {
            cursed_obj.placed = true;
            transform.translation = spot_pos + Vec3::new(0.0, 0.35, 0.0);
        }
        onryo.stage += 1;
        onryo.stacks = (onryo.stacks - tables.onryo.stack_reward_correct).max(0.0);
        let sequence_len = placement
            .as_ref()
            .map(|p| p.onryo_ritual_positions.len() as u8)
            .or_else(|| house_layout.as_ref().map(|l| l.exorcism.onryo_ritual_positions.len() as u8))
            .unwrap_or(tables.onryo.default_ritual_count);
        if onryo.stage >= sequence_len {
            if ghost_type.active == GhostType::Onryo {
                status.state = ExorcismState::Complete;
            } else {
                status.state = ExorcismState::Failed;
            }
        }
    } else {
        onryo.stacks += tables.onryo.stack_penalty_wrong;
    }
}
