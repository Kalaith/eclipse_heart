//! Top-level game coordinator.

use macroquad::rand::gen_range;

use crate::data::{GameContent, UiText};
use crate::engine::{AiController, MatchEngine};
use crate::screens::{BattleScreen, DeckBuilderScreen, MenuScreen, ScreenAction, SetupScreen};
use crate::state::{
    AppScreen, AppState, BoosterCardGrant, CollectionCardKind, MatchState,
};

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

        Self {
            state: AppState::new(ui_text, content),
            menu_screen: MenuScreen::new(),
            setup_screen: SetupScreen::new(),
            deck_builder_screen: DeckBuilderScreen::new(),
            battle_screen: BattleScreen::new(),
        }
    }

    pub fn update(&mut self) {
        let screen_action = match self.state.screen {
            AppScreen::Menu => self.menu_screen.update(&self.state),
            AppScreen::Setup => self.setup_screen.update(&self.state),
            AppScreen::DeckBuilder => self.deck_builder_screen.update(&self.state),
            AppScreen::Battle => self.battle_screen.update(&self.state),
        };

        match screen_action {
            ScreenAction::None => {}
            ScreenAction::OpenSetup => {
                self.state.screen = AppScreen::Setup;
            }
            ScreenAction::OpenDeckBuilder => {
                self.state
                    .saves
                    .decks
                    .ensure_active_support_deck(&self.state.content.starter_loadouts);
                self.state.screen = AppScreen::DeckBuilder;
            }
            ScreenAction::DeckBuilderOpenBooster => {
                self.state.last_opened_booster = open_booster(&self.state.content);
                for grant in &self.state.last_opened_booster {
                    self.state
                        .saves
                        .collection
                        .add_owned(grant.kind, &grant.id, 1);
                }
                let _ = self.state.persistence.save_all(&self.state.saves);
            }
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
            ScreenAction::DeckBuilderLoadStarter { loadout_index } => {
                if let Some(starter) = self.state.content.starter_loadouts.get(loadout_index) {
                    self.state.saves.decks.load_starter_into_active(starter);
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::DeckBuilderAddCard { card_id } => {
                if self
                    .state
                    .saves
                    .decks
                    .add_card(
                        &card_id,
                        &self.state.content.deck_rules,
                        &self.state.saves.collection,
                    )
                {
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::DeckBuilderRemoveCard { card_id } => {
                if self.state.saves.decks.remove_card(&card_id) {
                    let _ = self.state.persistence.save_all(&self.state.saves);
                }
            }
            ScreenAction::StartConfiguredBattle => {
                self.state.match_state = Some(MatchState::from_setup(
                    &self.state.content,
                    &self.state.setup,
                ));
                self.state.screen = AppScreen::Battle;
            }
            ScreenAction::BackToMenu => {
                self.state.screen = AppScreen::Menu;
            }
            ScreenAction::ApplyMatchAction(action) => {
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
        }

        if self.state.screen == AppScreen::Battle {
            if let Some(match_state) = &mut self.state.match_state {
                if let Some(action) = AiController::choose_action(match_state) {
                    MatchEngine::apply_action(match_state, action);
                }
            }
        }
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
