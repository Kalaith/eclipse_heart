//! Top-level game coordinator.

mod booster;
mod escape_menu;

use macroquad::input::{is_key_pressed, show_mouse, KeyCode};
use macroquad::miniquad::window::quit;
use macroquad::window::{request_new_screen_size, set_fullscreen};

use self::booster::open_booster;
use crate::data::{GameContent, UiText};
use crate::engine::{AiController, MatchEngine};
use crate::screens::{
    BattleScreen, CampaignHubScreen, CampaignMenuScreen, DeckBuilderScreen, MenuScreen,
    ScreenAction, SetupScreen,
};
use crate::state::{
    AppScreen, AppState, BattleContext, DeckPreset, DecksSave, MatchPhase, MatchSetup, MatchState,
    PlayerId,
};
use crate::ui::card_widgets::{clear_action_buttons, render_action_buttons};

pub struct Game {
    state: AppState,
    menu_screen: MenuScreen,
    campaign_menu_screen: CampaignMenuScreen,
    campaign_hub_screen: CampaignHubScreen,
    setup_screen: SetupScreen,
    deck_builder_screen: DeckBuilderScreen,
    battle_screen: BattleScreen,
}

impl Game {
    pub async fn new() -> Self {
        let ui_text = UiText::load_async().await.unwrap_or_else(|error| {
            panic!("failed to load UI text: {error}");
        });
        let content = GameContent::load_async().await.unwrap_or_else(|error| {
            panic!("failed to load game content: {error}");
        });
        let state = AppState::new(ui_text, content).await;
        apply_window_settings(&state);
        show_mouse(true);

        Self {
            state,
            menu_screen: MenuScreen::new(),
            campaign_menu_screen: CampaignMenuScreen::new(),
            campaign_hub_screen: CampaignHubScreen::new(),
            setup_screen: SetupScreen::new(),
            deck_builder_screen: DeckBuilderScreen::new(),
            battle_screen: BattleScreen::new(),
        }
    }

    pub fn update(&mut self) {
        show_mouse(true);
        if is_key_pressed(KeyCode::Escape) {
            self.state.escape_menu_open = !self.state.escape_menu_open;
            return;
        }
        if self.state.escape_menu_open {
            let screen_action = self.escape_menu_action();
            self.handle_screen_action(screen_action);
            return;
        }
        let screen_action = self.current_screen_action();
        self.handle_screen_action(screen_action);
        self.run_battle_ai_turn();
    }

    pub fn draw(&self) {
        match self.state.screen {
            AppScreen::Menu => self.menu_screen.draw(&self.state),
            AppScreen::CampaignMenu => self.campaign_menu_screen.draw(&self.state),
            AppScreen::CampaignHub => self.campaign_hub_screen.draw(&self.state),
            AppScreen::Setup => self.setup_screen.draw(&self.state),
            AppScreen::DeckBuilder => self.deck_builder_screen.draw(&self.state),
            AppScreen::Battle => self.battle_screen.draw(&self.state),
        }
        if self.state.escape_menu_open {
            clear_action_buttons();
            self.draw_escape_menu();
        }
        render_action_buttons();
    }
}

impl Game {
    fn current_screen_action(&mut self) -> ScreenAction {
        match self.state.screen {
            AppScreen::Menu => self.menu_screen.update(&self.state),
            AppScreen::CampaignMenu => self.campaign_menu_screen.update(&self.state),
            AppScreen::CampaignHub => self.campaign_hub_screen.update(&self.state),
            AppScreen::Setup => self.setup_screen.update(&self.state),
            AppScreen::DeckBuilder => self.deck_builder_screen.update(&self.state),
            AppScreen::Battle => self.battle_screen.update(&self.state),
        }
    }

    fn handle_screen_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::None => {}
            ScreenAction::OpenCampaignMenu
            | ScreenAction::OpenSetup
            | ScreenAction::OpenDeckBuilder
            | ScreenAction::ToggleEscapeMenu
            | ScreenAction::EscapeMenuSave
            | ScreenAction::EscapeMenuExitToMainMenu
            | ScreenAction::BackToMenu
            | ScreenAction::ToggleFullscreenMode
            | ScreenAction::ExitGame => {
                self.handle_navigation_action(action);
            }
            ScreenAction::DeckBuilderOpenBooster
            | ScreenAction::DeckBuilderCreateEmptyDeck
            | ScreenAction::DeckBuilderSelectDeck { .. }
            | ScreenAction::DeckBuilderRenameSelectedDeck { .. }
            | ScreenAction::DeckBuilderDuplicateSelectedDeck
            | ScreenAction::DeckBuilderDeleteSelectedDeck
            | ScreenAction::DeckBuilderImportDeckCode { .. }
            | ScreenAction::DeckBuilderSaveMetadata { .. }
            | ScreenAction::DeckBuilderUndoSelectedDeckChange
            | ScreenAction::DeckBuilderResetSelectedDeckToTemplate
            | ScreenAction::DeckBuilderSetRosterSlot { .. }
            | ScreenAction::DeckBuilderCreateDeckFromTemplate { .. }
            | ScreenAction::DeckBuilderAddCard { .. }
            | ScreenAction::DeckBuilderRemoveCard { .. } => {
                self.handle_deck_builder_action(action);
            }
            ScreenAction::SetupCyclePlayerAMgMain
            | ScreenAction::SetupCyclePlayerAMgSupports
            | ScreenAction::SetupCyclePlayerABaddieMain
            | ScreenAction::SetupCyclePlayerABaddieSupports
            | ScreenAction::SetupCyclePlayerBMgMain
            | ScreenAction::SetupCyclePlayerBMgSupports
            | ScreenAction::SetupCyclePlayerBBaddieMain
            | ScreenAction::SetupCyclePlayerBBaddieSupports
            | ScreenAction::SetupSelectMain { .. }
            | ScreenAction::SetupUseSelectedDeck { .. }
            | ScreenAction::SetupClearAssignedDeck { .. }
            | ScreenAction::SetupSelectSupportPair { .. } => {
                self.handle_setup_action(action);
            }
            ScreenAction::CampaignSelectRun { .. }
            | ScreenAction::CampaignToggleSupportSelection { .. }
            | ScreenAction::CampaignStartNewRun
            | ScreenAction::CampaignContinueRun
            | ScreenAction::CampaignAbandonRun
            | ScreenAction::CampaignStartEncounter => self.handle_campaign_action(action),
            ScreenAction::StartConfiguredBattle => self.start_configured_battle(),
            ScreenAction::ApplyMatchAction(action) => self.handle_match_action(action),
        }
    }

    fn handle_navigation_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::ToggleEscapeMenu => {
                self.state.escape_menu_open = !self.state.escape_menu_open;
            }
            ScreenAction::EscapeMenuSave => {
                let _ = self.state.persistence.save_all(&self.state.saves);
                self.state.escape_menu_open = false;
            }
            ScreenAction::EscapeMenuExitToMainMenu => {
                self.state.escape_menu_open = false;
                self.handle_navigation_action(ScreenAction::BackToMenu);
            }
            ScreenAction::OpenCampaignMenu => {
                self.state.escape_menu_open = false;
                self.state.screen = AppScreen::CampaignMenu;
            }
            ScreenAction::OpenSetup => {
                self.state.escape_menu_open = false;
                let available_deck_ids = self
                    .state
                    .saves
                    .decks
                    .support_decks
                    .iter()
                    .map(|deck| deck.id.clone())
                    .collect::<Vec<_>>();
                self.state
                    .setup
                    .clear_missing_support_deck_assignments(&available_deck_ids);
                self.state.screen = AppScreen::Setup;
            }
            ScreenAction::OpenDeckBuilder => {
                self.state.escape_menu_open = false;
                let magical_girl_ids = self
                    .state
                    .content
                    .magical_girls
                    .iter()
                    .map(|entry| entry.id.clone())
                    .collect::<Vec<_>>();
                let baddie_ids = self
                    .state
                    .content
                    .baddies
                    .iter()
                    .map(|entry| entry.id.clone())
                    .collect::<Vec<_>>();
                self.state
                    .saves
                    .decks
                    .ensure_valid_support_decks(&magical_girl_ids, &baddie_ids);
                self.state.screen = AppScreen::DeckBuilder;
            }
            ScreenAction::BackToMenu => {
                self.state.escape_menu_open = false;
                if self.state.screen == AppScreen::Battle
                    && matches!(self.state.battle_context, BattleContext::Campaign { .. })
                {
                    self.state.match_state = None;
                    self.state.battle_context = BattleContext::Skirmish;
                }
                self.state.screen = AppScreen::Menu;
            }
            ScreenAction::ToggleFullscreenMode => {
                self.state.saves.settings.fullscreen = !self.state.saves.settings.fullscreen;
                apply_window_settings(&self.state);
                let _ = self.state.persistence.save_all(&self.state.saves);
            }
            ScreenAction::ExitGame => {
                self.state.escape_menu_open = false;
                quit();
            }
            _ => {}
        }
    }

    fn handle_setup_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::SetupCyclePlayerAMgMain => {
                self.state.setup.cycle_player_a_mg_main(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerAMgSupports => {
                self.state
                    .setup
                    .cycle_player_a_mg_supports(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerABaddieMain => {
                self.state
                    .setup
                    .cycle_player_a_baddie_main(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerABaddieSupports => {
                self.state
                    .setup
                    .cycle_player_a_baddie_supports(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerBMgMain => {
                self.state.setup.cycle_player_b_mg_main(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerBMgSupports => {
                self.state
                    .setup
                    .cycle_player_b_mg_supports(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerBBaddieMain => {
                self.state
                    .setup
                    .cycle_player_b_baddie_main(&self.state.content);
            }
            ScreenAction::SetupCyclePlayerBBaddieSupports => {
                self.state
                    .setup
                    .cycle_player_b_baddie_supports(&self.state.content);
            }
            ScreenAction::SetupSelectMain {
                player,
                is_magical_girl_side,
                main_index,
            } => {
                self.state.setup.select_main(
                    &self.state.content,
                    player,
                    is_magical_girl_side,
                    main_index,
                );
            }
            ScreenAction::SetupSelectSupportPair {
                player,
                is_magical_girl_side,
                pair_index,
            } => {
                self.state.setup.select_support_pair(
                    &self.state.content,
                    player,
                    is_magical_girl_side,
                    pair_index,
                );
            }
            ScreenAction::SetupUseSelectedDeck { player } => {
                let selected_deck_id = self
                    .state
                    .saves
                    .decks
                    .selected_support_deck()
                    .map(|deck| deck.id.clone());
                self.state
                    .setup
                    .assign_support_deck(player, selected_deck_id);
            }
            ScreenAction::SetupClearAssignedDeck { player } => {
                self.state.setup.assign_support_deck(player, None);
            }
            _ => {}
        }
    }

    fn handle_campaign_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::CampaignSelectRun { run_id } => {
                if self.state.saves.campaigns.select_run(&run_id) {
                    self.state.campaign_notice = None;
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::CampaignToggleSupportSelection { character_id } => {
                if self
                    .state
                    .saves
                    .campaigns
                    .toggle_selected_magical_girl_support(&character_id)
                {
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::CampaignStartNewRun => {
                let Some(seed_deck) = self.campaign_seed_deck() else {
                    return;
                };
                if self
                    .state
                    .saves
                    .campaigns
                    .create_run(
                        &self.state.content.campaign,
                        &seed_deck,
                        self.state.ui_text.get("campaign_default_run_name"),
                    )
                    .is_some()
                {
                    self.state.campaign_notice = Some(format!(
                        "{} {}",
                        self.state.ui_text.get("campaign_notice_run_started"),
                        seed_deck.name
                    ));
                    self.state.screen = AppScreen::CampaignHub;
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::CampaignContinueRun => {
                if self.state.saves.campaigns.selected_run_is_in_progress() {
                    self.state.screen = AppScreen::CampaignHub;
                }
            }
            ScreenAction::CampaignAbandonRun => {
                if self.state.saves.campaigns.abandon_selected_run() {
                    self.state.campaign_notice = Some(
                        self.state
                            .ui_text
                            .get("campaign_notice_run_abandoned")
                            .to_owned(),
                    );
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::CampaignStartEncounter => self.start_campaign_encounter(),
            _ => {}
        }
    }

    fn handle_deck_builder_action(&mut self, action: ScreenAction) {
        let mut should_save = false;

        match action {
            ScreenAction::DeckBuilderOpenBooster => {
                self.state.last_opened_booster = open_booster(&self.state.content);
                for grant in &self.state.last_opened_booster {
                    self.state
                        .saves
                        .collection
                        .add_owned(grant.kind, &grant.id, 1);
                }
                should_save = true;
            }
            ScreenAction::DeckBuilderCreateEmptyDeck => {
                let magical_girl_ids = self
                    .state
                    .content
                    .magical_girls
                    .iter()
                    .map(|entry| entry.id.clone())
                    .collect::<Vec<_>>();
                let baddie_ids = self
                    .state
                    .content
                    .baddies
                    .iter()
                    .map(|entry| entry.id.clone())
                    .collect::<Vec<_>>();
                self.state.saves.decks.create_empty_deck(
                    self.state.ui_text.get("deck_builder_default_new_deck_name"),
                    &magical_girl_ids,
                    &baddie_ids,
                );
                should_save = true;
            }
            ScreenAction::DeckBuilderSelectDeck { deck_id } => {
                should_save = self.state.saves.decks.select_support_deck(&deck_id);
            }
            ScreenAction::DeckBuilderRenameSelectedDeck { name } => {
                should_save = self.state.saves.decks.rename_selected_deck(&name);
            }
            ScreenAction::DeckBuilderDuplicateSelectedDeck => {
                should_save = self
                    .state
                    .saves
                    .decks
                    .duplicate_selected_deck(self.state.ui_text.get("deck_builder_copy_suffix"))
                    .is_some();
            }
            ScreenAction::DeckBuilderDeleteSelectedDeck => {
                should_save = self.state.saves.decks.delete_selected_deck();
            }
            ScreenAction::DeckBuilderImportDeckCode { code } => {
                if let Ok(imported) = crate::state::import_deck_code(&code, &self.state.content) {
                    self.state.saves.decks.import_deck(
                        imported,
                        self.state.ui_text.get("deck_builder_imported_deck_name"),
                    );
                    should_save = true;
                }
            }
            ScreenAction::DeckBuilderSaveMetadata { notes, tags } => {
                should_save = self
                    .state
                    .saves
                    .decks
                    .update_selected_deck_metadata(&notes, &tags);
            }
            ScreenAction::DeckBuilderUndoSelectedDeckChange => {
                should_save = self.state.saves.decks.undo_selected_deck_change();
            }
            ScreenAction::DeckBuilderResetSelectedDeckToTemplate => {
                let Some(template_id) = self
                    .state
                    .saves
                    .decks
                    .selected_support_deck()
                    .and_then(|deck| deck.source_template_id.clone())
                else {
                    return;
                };
                if let Some(starter) = self
                    .state
                    .content
                    .starter_loadouts
                    .iter()
                    .find(|starter| starter.id == template_id)
                {
                    should_save = self
                        .state
                        .saves
                        .decks
                        .reset_selected_deck_to_template(starter);
                }
            }
            ScreenAction::DeckBuilderCreateDeckFromTemplate { loadout_index } => {
                if let Some(starter) = self.state.content.starter_loadouts.get(loadout_index) {
                    let magical_girl_ids = self
                        .state
                        .content
                        .magical_girls
                        .iter()
                        .map(|entry| entry.id.clone())
                        .collect::<Vec<_>>();
                    let baddie_ids = self
                        .state
                        .content
                        .baddies
                        .iter()
                        .map(|entry| entry.id.clone())
                        .collect::<Vec<_>>();
                    self.state.saves.decks.create_deck_from_template(
                        starter,
                        &magical_girl_ids,
                        &baddie_ids,
                        self.state.ui_text.get("deck_builder_copy_suffix"),
                    );
                    should_save = true;
                }
            }
            ScreenAction::DeckBuilderSetRosterSlot {
                is_magical_girl_side,
                slot_index,
                character_id,
            } => {
                should_save = self.state.saves.decks.set_roster_slot(
                    is_magical_girl_side,
                    slot_index,
                    &character_id,
                );
            }
            ScreenAction::DeckBuilderAddCard { card_id } => {
                should_save = self.state.saves.decks.add_card(
                    &card_id,
                    &self.state.content.deck_rules,
                    &self.state.saves.collection,
                );
            }
            ScreenAction::DeckBuilderRemoveCard { card_id } => {
                should_save = self.state.saves.decks.remove_card(&card_id);
            }
            _ => {}
        }

        if should_save {
            let _ = self.state.persistence.save_all(&self.state.saves);
        }
    }

    fn start_configured_battle(&mut self) {
        let player_a_deck = self
            .state
            .setup
            .player_a_support_deck_id
            .as_deref()
            .and_then(|deck_id| {
                self.state
                    .saves
                    .decks
                    .support_decks
                    .iter()
                    .find(|deck| deck.id == deck_id)
            })
            .map(|deck| deck.story_cards.clone())
            .unwrap_or_else(MatchState::default_player_a_support_deck);
        let player_b_deck = self
            .state
            .setup
            .player_b_support_deck_id
            .as_deref()
            .and_then(|deck_id| {
                self.state
                    .saves
                    .decks
                    .support_decks
                    .iter()
                    .find(|deck| deck.id == deck_id)
            })
            .map(|deck| deck.story_cards.clone())
            .unwrap_or_else(MatchState::default_player_b_support_deck);
        self.state.match_state = Some(MatchState::from_setup_with_options(
            &self.state.content,
            &self.state.setup,
            player_a_deck,
            player_b_deck,
            PlayerId::PlayerA,
        ));
        self.state.battle_context = BattleContext::Skirmish;
        self.state.screen = AppScreen::Battle;
    }

    fn start_campaign_encounter(&mut self) {
        if !self
            .state
            .saves
            .campaigns
            .selected_run_has_valid_support_pair()
        {
            return;
        }
        let Some(selected_run) = self.state.saves.campaigns.selected_run() else {
            return;
        };
        let run_id = selected_run.id.clone();
        let Some(node) = self
            .state
            .content
            .campaign
            .node(&selected_run.current_node_id)
        else {
            return;
        };
        let Some(encounter) = self.state.content.campaign.encounter(&node.encounter_id) else {
            return;
        };
        let Some(enemy_loadout) = self
            .state
            .content
            .starter_loadouts
            .iter()
            .find(|loadout| loadout.id == encounter.enemy_loadout_id)
        else {
            return;
        };

        let setup = MatchSetup::from_player_deck_and_enemy_loadout(
            &self.state.content,
            &selected_run.player_deck,
            &selected_run.selected_magical_girl_support_ids,
            enemy_loadout,
        );
        self.state.match_state = Some(MatchState::from_setup_with_options(
            &self.state.content,
            &setup,
            selected_run.player_deck.story_cards.clone(),
            enemy_loadout.support_deck.clone(),
            PlayerId::PlayerA,
        ));
        self.state.battle_context = BattleContext::Campaign {
            run_id,
            node_id: node.id.clone(),
        };
        self.state.screen = AppScreen::Battle;
    }

    fn handle_match_action(&mut self, action: crate::engine::MatchAction) {
        if let Some(match_state) = &mut self.state.match_state {
            let was_finished = match_state.phase == MatchPhase::Finished;
            MatchEngine::apply_action(match_state, action);
            if !was_finished && match_state.phase == MatchPhase::Finished {
                self.state.saves.profile.total_matches_played += 1;
                if match_state.winner == Some(PlayerId::PlayerA) {
                    self.state.saves.profile.total_wins += 1;
                }
                let _ = self.state.persistence.save_all(&self.state.saves);
            }
        }
        self.resolve_finished_battle();
    }

    fn run_battle_ai_turn(&mut self) {
        if self.state.screen != AppScreen::Battle {
            return;
        }

        if let Some(match_state) = &mut self.state.match_state {
            if let Some(action) = AiController::choose_action(match_state) {
                MatchEngine::apply_action(match_state, action);
                self.resolve_finished_battle();
            }
        }
    }

    fn resolve_finished_battle(&mut self) {
        let Some(match_state) = self.state.match_state.as_ref() else {
            return;
        };
        if match_state.phase != MatchPhase::Finished {
            return;
        }

        let (run_id, node_id) = match &self.state.battle_context {
            BattleContext::Skirmish => return,
            BattleContext::Campaign { run_id, node_id } => (run_id.clone(), node_id.clone()),
        };
        let winner = match_state.winner;

        match winner {
            Some(PlayerId::PlayerA) => self.resolve_campaign_victory(&run_id, &node_id),
            Some(PlayerId::PlayerB) | None => self.resolve_campaign_defeat(&run_id, &node_id),
        }
    }

    fn resolve_campaign_victory(&mut self, run_id: &str, node_id: &str) {
        let reward_card = self
            .state
            .content
            .campaign
            .node(node_id)
            .and_then(|node| self.state.content.campaign.encounter(&node.encounter_id))
            .and_then(|encounter| encounter.reward_story_card_ids.first())
            .cloned();
        let reward_card_name = reward_card.as_deref().and_then(|card_id| {
            self.state
                .content
                .story_cards
                .iter()
                .find(|card| card.id == card_id)
                .map(|card| card.name.clone())
        });
        let campaign_finished = self.state.saves.campaigns.record_victory_for_run(
            run_id,
            &self.state.content.campaign,
            node_id,
            reward_card.as_deref(),
        );

        self.state.campaign_notice = match campaign_finished {
            Some(true) => Some(
                self.state
                    .ui_text
                    .get("campaign_notice_campaign_cleared")
                    .to_owned(),
            ),
            Some(false) => Some(match reward_card_name {
                Some(card_name) => format!(
                    "{} {}",
                    self.state.ui_text.get("campaign_notice_victory_reward"),
                    card_name
                ),
                None => self.state.ui_text.get("campaign_notice_victory").to_owned(),
            }),
            None => Some(self.state.ui_text.get("campaign_notice_victory").to_owned()),
        };
        self.state.match_state = None;
        self.state.battle_context = BattleContext::Skirmish;
        self.state.screen = if campaign_finished == Some(true) {
            AppScreen::CampaignMenu
        } else {
            AppScreen::CampaignHub
        };
        let _ = self.state.persistence.save_all(&self.state.saves);
    }

    fn resolve_campaign_defeat(&mut self, run_id: &str, node_id: &str) {
        if self.state.saves.campaigns.record_defeat_for_run(
            run_id,
            &self.state.content.campaign,
            node_id,
        ) {
            self.state.campaign_notice =
                Some(self.state.ui_text.get("campaign_notice_defeat").to_owned());
            let _ = self.state.persistence.save_all(&self.state.saves);
        }
        self.state.match_state = None;
        self.state.battle_context = BattleContext::Skirmish;
        self.state.screen = AppScreen::CampaignMenu;
    }

    fn campaign_seed_deck(&self) -> Option<DeckPreset> {
        if let Some(selected) = self.state.saves.decks.selected_support_deck() {
            return Some(selected.clone());
        }

        let starter = self.state.content.starter_loadouts.first()?;
        let magical_girl_ids = self
            .state
            .content
            .magical_girls
            .iter()
            .map(|entry| entry.id.clone())
            .collect::<Vec<_>>();
        let baddie_ids = self
            .state
            .content
            .baddies
            .iter()
            .map(|entry| entry.id.clone())
            .collect::<Vec<_>>();
        let mut decks = DecksSave::default();
        decks.create_deck_from_template(
            starter,
            &magical_girl_ids,
            &baddie_ids,
            self.state.ui_text.get("deck_builder_copy_suffix"),
        );
        decks.selected_support_deck().cloned()
    }
}

fn apply_window_settings(state: &AppState) {
    let settings = &state.saves.settings;
    set_fullscreen(settings.fullscreen);
    if !settings.fullscreen {
        request_new_screen_size(settings.window_width as f32, settings.window_height as f32);
    }
}
