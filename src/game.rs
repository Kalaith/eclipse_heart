//! Top-level game coordinator.

use macroquad::miniquad::window::quit;
use macroquad::rand::gen_range;
use macroquad::window::{request_new_screen_size, set_fullscreen};

use crate::data::{GameContent, UiText};
use crate::engine::{AiController, MatchEngine};
use crate::screens::{BattleScreen, DeckBuilderScreen, MenuScreen, ScreenAction, SetupScreen};
use crate::state::{AppScreen, AppState, BoosterCardGrant, CollectionCardKind, MatchState};

pub struct Game {
    state: AppState,
    menu_screen: MenuScreen,
    setup_screen: SetupScreen,
    deck_builder_screen: DeckBuilderScreen,
    battle_screen: BattleScreen,
}

impl Game {
    pub async fn new() -> Self {
        let ui_text = UiText::load().unwrap_or_default();
        let content = GameContent::load().unwrap_or_default();
        let state = AppState::new(ui_text, content);
        apply_window_settings(&state);

        Self {
            state,
            menu_screen: MenuScreen::new(),
            setup_screen: SetupScreen::new(),
            deck_builder_screen: DeckBuilderScreen::new(),
            battle_screen: BattleScreen::new(),
        }
    }

    pub fn update(&mut self) {
        let screen_action = self.current_screen_action();
        self.handle_screen_action(screen_action);
        self.run_battle_ai_turn();
    }

    pub fn draw(&self) {
        match self.state.screen {
            AppScreen::Menu => self.menu_screen.draw(&self.state),
            AppScreen::Setup => self.setup_screen.draw(&self.state),
            AppScreen::DeckBuilder => self.deck_builder_screen.draw(&self.state),
            AppScreen::Battle => self.battle_screen.draw(&self.state),
        }
    }
}

impl Game {
    fn current_screen_action(&mut self) -> ScreenAction {
        match self.state.screen {
            AppScreen::Menu => self.menu_screen.update(&self.state),
            AppScreen::Setup => self.setup_screen.update(&self.state),
            AppScreen::DeckBuilder => self.deck_builder_screen.update(&self.state),
            AppScreen::Battle => self.battle_screen.update(&self.state),
        }
    }

    fn handle_screen_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::None => {}
            ScreenAction::OpenSetup
            | ScreenAction::OpenDeckBuilder
            | ScreenAction::BackToMenu
            | ScreenAction::ToggleWindowedMode
            | ScreenAction::ExitGame => {
                self.handle_navigation_action(action);
            }
            ScreenAction::DeckBuilderOpenBooster
            | ScreenAction::DeckBuilderCreateEmptyDeck
            | ScreenAction::DeckBuilderSelectDeck { .. }
            | ScreenAction::DeckBuilderRenameSelectedDeck { .. }
            | ScreenAction::DeckBuilderDuplicateSelectedDeck
            | ScreenAction::DeckBuilderDeleteSelectedDeck
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
            | ScreenAction::SetupSelectSupportPair { .. } => {
                self.handle_setup_action(action);
            }
            ScreenAction::StartConfiguredBattle => self.start_configured_battle(),
            ScreenAction::ApplyMatchAction(action) => self.handle_match_action(action),
        }
    }

    fn handle_navigation_action(&mut self, action: ScreenAction) {
        match action {
            ScreenAction::OpenSetup => {
                self.state.screen = AppScreen::Setup;
            }
            ScreenAction::OpenDeckBuilder => {
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
                self.state.screen = AppScreen::Menu;
            }
            ScreenAction::ToggleWindowedMode => {
                self.state.saves.settings.fullscreen = !self.state.saves.settings.fullscreen;
                apply_window_settings(&self.state);
                let _ = self.state.persistence.save_all(&self.state.saves);
            }
            ScreenAction::ExitGame => {
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
        self.state.match_state = Some(MatchState::from_setup(
            &self.state.content,
            &self.state.setup,
        ));
        self.state.screen = AppScreen::Battle;
    }

    fn handle_match_action(&mut self, action: crate::engine::MatchAction) {
        if let Some(match_state) = &mut self.state.match_state {
            let was_finished = match_state.phase == crate::state::MatchPhase::Finished;
            MatchEngine::apply_action(match_state, action);
            if !was_finished && match_state.phase == crate::state::MatchPhase::Finished {
                self.state.saves.profile.total_matches_played += 1;
                if match_state.winner == Some(crate::state::PlayerId::PlayerA) {
                    self.state.saves.profile.total_wins += 1;
                }
                let _ = self.state.persistence.save_all(&self.state.saves);
            }
        }
    }

    fn run_battle_ai_turn(&mut self) {
        if self.state.screen != AppScreen::Battle {
            return;
        }

        if let Some(match_state) = &mut self.state.match_state {
            if let Some(action) = AiController::choose_action(match_state) {
                MatchEngine::apply_action(match_state, action);
            }
        }
    }
}

fn apply_window_settings(state: &AppState) {
    let settings = &state.saves.settings;
    set_fullscreen(settings.fullscreen);
    if !settings.fullscreen {
        request_new_screen_size(settings.window_width as f32, settings.window_height as f32);
    }
}

fn open_booster(content: &GameContent) -> Vec<BoosterCardGrant> {
    let mut pool = Vec::new();

    for entry in &content.magical_girls {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::MagicalGirl,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    for entry in &content.baddies {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::Baddie,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    for entry in &content.story_cards {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::StoryCard,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    let mut results = Vec::new();
    for _ in 0..10 {
        let index = gen_range(0, pool.len() as i32) as usize;
        results.push(pool[index].clone());
    }
    results
}
