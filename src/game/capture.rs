use macroquad::input::show_mouse;

use crate::state::{AppScreen, CampaignSaveBundle};

use super::Game;

impl Game {
    pub fn update_for_capture(&mut self) {
        show_mouse(true);
        let _ = self.current_screen_action();
    }

    pub fn prepare_capture_screen(&mut self, screen_name: &str) -> bool {
        self.state.escape_menu_open = false;
        self.menu_screen.set_settings_open(false);

        match screen_name {
            "title_screen" | "title" | "menu" => {
                self.state.screen = AppScreen::Menu;
                true
            }
            "settings_screen" | "settings" => {
                self.state.screen = AppScreen::Menu;
                self.menu_screen.set_settings_open(true);
                true
            }
            "deck_builder" => {
                self.ensure_capture_deck();
                self.state.screen = AppScreen::DeckBuilder;
                true
            }
            "match_setup" | "setup" => {
                self.ensure_capture_deck();
                self.state.screen = AppScreen::Setup;
                true
            }
            "battle" => {
                self.ensure_capture_deck();
                self.start_configured_battle();
                true
            }
            "escape_menu" => {
                self.ensure_capture_deck();
                self.start_configured_battle();
                self.state.escape_menu_open = true;
                true
            }
            "campaign_menu" => {
                self.ensure_capture_campaign_run();
                self.state.screen = AppScreen::CampaignMenu;
                true
            }
            "campaign_hub" => {
                self.ensure_capture_campaign_run();
                self.state.screen = AppScreen::CampaignHub;
                true
            }
            "campaign_battle" => {
                self.ensure_capture_campaign_run();
                self.start_campaign_encounter();
                true
            }
            _ => false,
        }
    }

    fn ensure_capture_deck(&mut self) {
        if self.state.saves.decks.selected_support_deck().is_some() {
            return;
        }
        let Some(starter) = self.state.content.starter_loadouts.first() else {
            return;
        };
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
    }

    fn ensure_capture_campaign_run(&mut self) {
        self.state.saves.campaigns = CampaignSaveBundle::default();
        self.state.campaign_notice = None;
        if self
            .state
            .saves
            .campaigns
            .selected_run_has_valid_support_pair()
        {
            return;
        }
        let Some(seed_deck) = self.campaign_seed_deck() else {
            return;
        };
        if !self.state.saves.campaigns.selected_run_is_in_progress() {
            let _ = self.state.saves.campaigns.create_run(
                &self.state.content.campaign,
                &seed_deck,
                self.state.ui_text.get("campaign_default_run_name"),
            );
        }
        let Some(run) = self.state.saves.campaigns.selected_run() else {
            return;
        };
        let support_ids = run
            .player_deck
            .magical_girl_roster
            .iter()
            .skip(1)
            .take(2)
            .cloned()
            .collect::<Vec<_>>();
        let _ = self
            .state
            .saves
            .campaigns
            .update_selected_magical_girl_supports(&support_ids);
    }
}
