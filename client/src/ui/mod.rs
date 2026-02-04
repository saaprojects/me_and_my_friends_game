use crate::prelude::*;

pub mod hud;
pub mod lobby;

#[derive(Component)]
pub struct StartScreenRoot;

#[derive(Component)]
pub struct RoleSelectRoot;

#[derive(Component)]
pub struct GhostDetailRoot;

#[derive(Component)]
pub struct InvestigatorDetailRoot;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct ToolText;

#[derive(Component)]
pub struct EmfText;

#[derive(Component)]
pub struct SpiritboxText;

#[derive(Component)]
pub struct JournalEmfText;

#[derive(Component)]
pub struct JournalSpiritText;

#[derive(Component)]
pub struct JournalGuessText;

#[derive(Component)]
pub struct JournalSection;

#[derive(Component)]
pub struct PuzzleTitleText;

#[derive(Component)]
pub struct PuzzleStatusText;

#[derive(Component)]
pub struct PuzzleDetailText;

#[derive(Component)]
pub struct GhostHudRoot;

#[derive(Component)]
pub struct GhostAbilityText;

#[derive(Component)]
pub struct JournalSelectSpiritButton;

#[derive(Component)]
pub struct JournalSelectBansheeButton;

#[derive(Component)]
pub struct JournalSelectOnryoButton;

#[derive(Component)]
pub struct JournalConfirmButton;

#[derive(Component)]
pub struct JournalConfirmText;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct GhostSelectButton;

#[derive(Component)]
pub struct InvestigatorSelectButton;

#[derive(Component)]
pub struct SpiritGhostButton;

#[derive(Component)]
pub struct BansheeGhostButton;

#[derive(Component)]
pub struct OnryoGhostButton;

#[derive(Component)]
pub struct StartScreenButton;

#[derive(Component)]
pub struct BeginHauntButton;

#[derive(Component)]
pub struct BeginInvestigationButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (lobby::setup_menu, hud::setup_hud))
            .add_systems(
                Update,
                (
                    lobby::handle_menu_toggle,
                    lobby::handle_menu_interactions,
                    lobby::sync_start_screen_visibility,
                    lobby::sync_role_select_visibility,
                    lobby::sync_ghost_detail_visibility,
                    lobby::sync_investigator_detail_visibility,
                    lobby::sync_menu_styles,
                    lobby::sync_role_select_hover,
                    lobby::update_cursor_lock,
                    hud::sync_hud_visibility,
                    hud::sync_ghost_hud_visibility,
                    hud::handle_journal_toggle,
                    hud::handle_journal_interactions,
                    hud::sync_journal_panel_visibility,
                    hud::sync_journal_visibility,
                    hud::sync_journal_styles,
                    hud::sync_hud_text,
                ),
            );
    }
}
