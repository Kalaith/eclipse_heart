//! Campaign menu screen.

use macroquad::prelude::*;

use crate::screens::ScreenAction;
use crate::state::{AppState, CampaignRunSave, CampaignRunStatus};
use crate::ui::card_widgets::{action_button, point_in_rect};
use crate::ui::core::{
    draw_background_texture, draw_focus_panel, draw_screen_scrim, draw_soft_panel, MG_BLUE,
    PRIORITY_GOLD, TEXT_MUTED,
};
use crate::ui::layout::UiLayout;
use macroquad_toolkit::ui::draw_ui_text;

pub struct CampaignMenuScreen;

impl Default for CampaignMenuScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl CampaignMenuScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, state: &AppState) -> ScreenAction {
        let ui = UiLayout::current();
        let mouse = mouse_position();

        for slot in campaign_slot_targets(state) {
            if point_in_rect(slot.rect, mouse) && is_mouse_button_pressed(MouseButton::Left) {
                return ScreenAction::CampaignSelectRun {
                    run_id: slot.run_id.clone(),
                };
            }
        }

        if action_button(
            ui.rect(1880.0, 760.0, 520.0, 78.0),
            state.ui_text.get("campaign_new_run"),
        ) {
            return ScreenAction::CampaignStartNewRun;
        }
        if state.saves.campaigns.selected_run_is_in_progress()
            && action_button(
                ui.rect(1880.0, 856.0, 520.0, 78.0),
                state.ui_text.get("campaign_continue_run"),
            )
        {
            return ScreenAction::CampaignContinueRun;
        }
        if state.saves.campaigns.selected_run_is_in_progress()
            && action_button(
                ui.rect(1880.0, 952.0, 520.0, 78.0),
                state.ui_text.get("campaign_abandon_run"),
            )
        {
            return ScreenAction::CampaignAbandonRun;
        }
        if action_button(
            ui.rect(1880.0, 1090.0, 520.0, 78.0),
            state.ui_text.get("battle_back_to_menu"),
        ) {
            return ScreenAction::BackToMenu;
        }
        ScreenAction::None
    }

    pub fn draw(&self, state: &AppState) {
        let ui = UiLayout::current();
        if let Some(background) = state.assets.ui_background("campaign") {
            draw_background_texture(background, Color::new(1.0, 1.0, 1.0, 0.30));
        }
        draw_screen_scrim(0.52);
        draw_focus_panel(ui.rect(56.0, 56.0, 1700.0, 1240.0), MG_BLUE);
        draw_focus_panel(ui.rect(1830.0, 690.0, 620.0, 530.0), PRIORITY_GOLD);

        draw_ui_text(
            state.ui_text.get("campaign_menu_title"),
            ui.x(80.0),
            ui.y(110.0),
            ui.font(72.0),
            WHITE,
        );
        draw_ui_text(
            state.ui_text.get("campaign_menu_subtitle"),
            ui.x(80.0),
            ui.y(164.0),
            ui.font(30.0),
            TEXT_MUTED,
        );
        draw_ui_text(
            &state.content.campaign.description,
            ui.x(80.0),
            ui.y(220.0),
            ui.font(26.0),
            GOLD,
        );

        draw_ui_text(
            state.ui_text.get("campaign_slots_label"),
            ui.x(80.0),
            ui.y(320.0),
            ui.font(30.0),
            WHITE,
        );

        if state.saves.campaigns.runs.is_empty() {
            draw_soft_panel(ui.x(80.0), ui.y(356.0), ui.w(1600.0), ui.h(104.0), MG_BLUE);
            draw_ui_text(
                state.ui_text.get("campaign_slot_empty"),
                ui.x(116.0),
                ui.y(420.0),
                ui.font(26.0),
                TEXT_MUTED,
            );
        } else {
            for slot in campaign_slot_targets(state) {
                self.draw_slot(state, &slot);
            }
        }

        if let Some(run) = state.saves.campaigns.selected_run() {
            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("campaign_selected_slot_label"),
                    run.name
                ),
                ui.x(80.0),
                ui.y(1110.0),
                ui.font(28.0),
                WHITE,
            );
            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("campaign_status_label"),
                    run_status_label(state, &run.status)
                ),
                ui.x(80.0),
                ui.y(1148.0),
                ui.font(24.0),
                TEXT_MUTED,
            );
            draw_ui_text(
                &format!(
                    "{}: {}",
                    state.ui_text.get("campaign_current_node_label"),
                    run.current_node_id
                ),
                ui.x(80.0),
                ui.y(1184.0),
                ui.font(24.0),
                TEXT_MUTED,
            );
        }

        if let Some(message) = &state.campaign_notice {
            draw_ui_text(message, ui.x(80.0), ui.y(1260.0), ui.font(24.0), SKYBLUE);
        }

        let completed_runs = state
            .saves
            .campaigns
            .runs
            .iter()
            .filter(|run| run.status == CampaignRunStatus::Won)
            .count();
        draw_ui_text(
            &format!(
                "{}: {}",
                state.ui_text.get("campaign_completed_runs_label"),
                completed_runs
            ),
            ui.x(80.0),
            ui.y(1306.0),
            ui.font(24.0),
            TEXT_MUTED,
        );
    }

    fn draw_slot(&self, state: &AppState, slot: &CampaignSlotTarget<'_>) {
        let ui = UiLayout::current();
        let outline = if slot.selected {
            PRIORITY_GOLD
        } else if slot.hovered {
            WHITE
        } else {
            GRAY
        };

        draw_soft_panel(slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h, DARKGRAY);
        if let Some(texture) = slot
            .run
            .player_deck
            .magical_girl_roster
            .first()
            .and_then(|character_id| state.assets.portrait(character_id))
        {
            draw_texture_ex(
                texture,
                slot.rect.x + ui.w(12.0),
                slot.rect.y + ui.h(10.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(ui.w(88.0), ui.h(84.0))),
                    ..Default::default()
                },
            );
        }
        draw_rectangle_lines(
            slot.rect.x,
            slot.rect.y,
            slot.rect.w,
            slot.rect.h,
            3.0,
            outline,
        );
        draw_ui_text(
            &slot.run.name,
            slot.rect.x + ui.w(116.0),
            slot.rect.y + ui.h(36.0),
            ui.font(24.0),
            WHITE,
        );
        draw_ui_text(
            &format!(
                "{}  {} {}/{}",
                run_status_label(state, &slot.run.status),
                state.ui_text.get("campaign_progress_label"),
                slot.run.completed_node_ids.len(),
                state.content.campaign.nodes.len()
            ),
            slot.rect.x + ui.w(116.0),
            slot.rect.y + ui.h(70.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
        draw_ui_text(
            &format!(
                "{}: {}",
                state.ui_text.get("campaign_deck_label"),
                slot.run.player_deck.name
            ),
            slot.rect.x + ui.w(560.0),
            slot.rect.y + ui.h(36.0),
            ui.font(20.0),
            GOLD,
        );
        draw_ui_text(
            &format!(
                "{}: {}",
                state.ui_text.get("campaign_current_node_label"),
                slot.run.current_node_id
            ),
            slot.rect.x + ui.w(560.0),
            slot.rect.y + ui.h(70.0),
            ui.font(18.0),
            TEXT_MUTED,
        );
    }
}

struct CampaignSlotTarget<'a> {
    run_id: String,
    rect: Rect,
    run: &'a CampaignRunSave,
    selected: bool,
    hovered: bool,
}

fn campaign_slot_targets<'a>(state: &'a AppState) -> Vec<CampaignSlotTarget<'a>> {
    let ui = UiLayout::current();
    let mouse = mouse_position();
    let selected_run_id = state
        .saves
        .campaigns
        .selected_run()
        .map(|run| run.id.as_str());

    state
        .saves
        .campaigns
        .runs
        .iter()
        .enumerate()
        .map(|(index, run)| {
            let rect = ui.rect(80.0, 356.0 + index as f32 * 120.0, 1600.0, 104.0);
            CampaignSlotTarget {
                run_id: run.id.clone(),
                rect,
                run,
                selected: selected_run_id == Some(run.id.as_str()),
                hovered: point_in_rect(rect, mouse),
            }
        })
        .collect()
}

fn run_status_label<'a>(state: &'a AppState, status: &CampaignRunStatus) -> &'a str {
    match status {
        CampaignRunStatus::InProgress => state.ui_text.get("campaign_status_in_progress"),
        CampaignRunStatus::Won => state.ui_text.get("campaign_status_won"),
        CampaignRunStatus::Lost => state.ui_text.get("campaign_status_lost"),
    }
}
