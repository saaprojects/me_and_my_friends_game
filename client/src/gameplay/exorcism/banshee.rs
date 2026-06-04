use crate::prelude::*;

use crate::core::{GhostType, GhostTypeState, InputMap, JournalState, MenuState, Role, RoleState};
use crate::gameplay::exorcism::tables::ExorcismTables;
use crate::gameplay::exorcism::{ExorcismState, ExorcismStatus};
use crate::gameplay::map::ExorcismPlacement;
use super::rules;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BansheeNodeColor {
    Violet,
    Amber,
    Teal,
}

impl BansheeNodeColor {
    pub fn palette(count: usize) -> Vec<Self> {
        [Self::Violet, Self::Amber, Self::Teal]
            .into_iter()
            .take(count.clamp(1, 3))
            .collect()
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Violet => "Violet",
            Self::Amber => "Amber",
            Self::Teal => "Teal",
        }
    }

    pub fn base_color(self) -> Color {
        match self {
            Self::Violet => Color::srgb(0.8, 0.38, 0.95),
            Self::Amber => Color::srgb(0.95, 0.72, 0.25),
            Self::Teal => Color::srgb(0.25, 0.85, 0.8),
        }
    }

    pub fn emissive(self) -> Color {
        match self {
            Self::Violet => Color::srgb(0.25, 0.1, 0.38),
            Self::Amber => Color::srgb(0.34, 0.2, 0.06),
            Self::Teal => Color::srgb(0.06, 0.24, 0.22),
        }
    }
}

#[derive(Component)]
pub struct BansheeAnchor {
    pub color: BansheeNodeColor,
}

#[derive(Resource, Clone, Debug)]
pub struct BansheeSequence {
    pub anchor_colors: Vec<BansheeNodeColor>,
    pub order: Vec<BansheeNodeColor>,
}

impl Default for BansheeSequence {
    fn default() -> Self {
        Self::for_anchor_count(3, &mut crate::core::GameRng::default())
    }
}

impl BansheeSequence {
    pub fn for_anchor_count(count: usize, rng: &mut crate::core::GameRng) -> Self {
        let anchor_colors = BansheeNodeColor::palette(count);
        let mut order = anchor_colors.clone();
        shuffle_colors(&mut order, rng.next_seed(count as u64));
        Self { anchor_colors, order }
    }

    pub fn reset_for_anchor_count(&mut self, count: usize, rng: &mut crate::core::GameRng) {
        *self = Self::for_anchor_count(count, rng);
    }

    pub fn sequence_len(&self) -> u8 {
        self.order.len() as u8
    }

    pub fn color_for_index(&self, index: usize) -> BansheeNodeColor {
        self.anchor_colors
            .get(index)
            .copied()
            .unwrap_or(BansheeNodeColor::Violet)
    }

    pub fn expected_color(&self, stage: u8) -> Option<BansheeNodeColor> {
        self.order.get(stage as usize).copied()
    }

    pub fn current_target_label(&self, stage: u8) -> &'static str {
        self.expected_color(stage)
            .map(BansheeNodeColor::label)
            .unwrap_or("Complete")
    }

    pub fn order_summary(&self) -> String {
        self.order
            .iter()
            .map(|color| color.label())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

fn shuffle_colors(colors: &mut [BansheeNodeColor], mut seed: u64) {
    for index in (1..colors.len()).rev() {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap_index = (seed % (index as u64 + 1)) as usize;
        colors.swap(index, swap_index);
    }
}

#[derive(Resource)]
pub struct BansheePuzzle {
    pub stage: u8,
    pub time_since_trigger: f32,
    pub failed_timer: f32,
}

impl Default for BansheePuzzle {
    fn default() -> Self {
        Self {
            stage: 0,
            time_since_trigger: 0.0,
            failed_timer: 0.0,
        }
    }
}

pub fn update_banshee_puzzle(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    role: Res<RoleState>,
    ghost_type: Res<GhostTypeState>,
    investigation: Res<super::InvestigationState>,
    tables: Res<ExorcismTables>,
    banshee_sequence: Res<BansheeSequence>,
    journal: Res<JournalState>,
    input: Res<InputMap>,
    placement: Option<Res<ExorcismPlacement>>,
    house_layout: Option<Res<crate::gameplay::map::HouseLayout>>,
    player: Query<&Transform, With<crate::gameplay::investigator::Player>>,
    anchors: Query<(&BansheeAnchor, &Transform)>,
    mut banshee: ResMut<BansheePuzzle>,
    mut status: ResMut<ExorcismStatus>,
) {
    if menu.open || journal.open || role.current != Role::Investigator {
        return;
    }
    let Some(puzzle_type) = investigation.guess else {
        return;
    };
    if !investigation.confirmed || puzzle_type != GhostType::Banshee {
        return;
    }

    let layout_sequence_len = placement
        .as_ref()
        .map(|p| p.banshee_anchors.len() as u8)
        .or_else(|| house_layout.as_ref().map(|l| l.exorcism.banshee_anchors.len() as u8))
        .unwrap_or_else(|| tables.banshee.sequence_len());
    let sequence_len = banshee_sequence
        .sequence_len()
        .max(layout_sequence_len)
        .max(1);
    banshee.time_since_trigger += time.delta_seconds();
    if banshee.failed_timer > 0.0 {
        banshee.failed_timer = (banshee.failed_timer - time.delta_seconds()).max(0.0);
        if banshee.failed_timer == 0.0 {
            banshee.stage = 0;
            banshee.time_since_trigger = 0.0;
        }
        status.state = ExorcismState::Failed;
        return;
    }

    status.state = ExorcismState::Stage(banshee.stage);
    status.stage = banshee.stage;
    status.progress = if sequence_len == 0 {
        0.0
    } else {
        banshee.stage as f32 / sequence_len as f32
    };
    status.stacks = 0.0;
    status.max_stacks = sequence_len as f32;

    if !keys.just_pressed(input.interact) {
        return;
    }

    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let mut closest: Option<(BansheeNodeColor, f32, Vec3)> = None;
    for (anchor, transform) in anchors.iter() {
        let distance = player_transform.translation.distance(transform.translation);
        if distance <= tables.banshee.interact_distance {
            if closest.is_none() || distance < closest.unwrap().1 {
                closest = Some((anchor.color, distance, transform.translation));
            }
        }
    }

    let Some((color, _, _pos)) = closest else {
        return;
    };

    let expected = banshee.stage;
    let order_ok = banshee_sequence.expected_color(expected) == Some(color);
    let timing_ok = if expected == 0 {
        true
    } else {
        let t = banshee.time_since_trigger;
        t >= tables.banshee.timing_min && t <= tables.banshee.timing_max
    };

    let next_state = rules::banshee_advance(banshee.stage, sequence_len, timing_ok, order_ok);
    match next_state {
        ExorcismState::Failed => {
            banshee.failed_timer = tables.banshee.fail_reset_seconds;
            banshee.stage = 0;
            status.state = ExorcismState::Failed;
        }
        ExorcismState::Complete => {
            if ghost_type.active == GhostType::Banshee {
                status.state = ExorcismState::Complete;
            } else {
                status.state = ExorcismState::Failed;
            }
            banshee.stage = sequence_len;
        }
        ExorcismState::Stage(next) => {
            banshee.stage = next;
            banshee.time_since_trigger = 0.0;
            status.state = ExorcismState::Stage(next);
        }
        _ => {}
    }
}
