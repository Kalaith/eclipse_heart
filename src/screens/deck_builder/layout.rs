use macroquad::prelude::Rect;

use crate::state::DeckViewMode;
use crate::ui::layout::UiLayout;

use super::types::{DeckBuilderTab, DeckTransferActionKind, DeckUtilityActionKind};

pub(super) fn browser_content_start_y() -> f32 {
    362.0
}

pub(super) fn browser_card_rect(
    view_mode: DeckViewMode,
    base_y: f32,
    row: usize,
    column: usize,
) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            ui.x(560.0 + column as f32 * 350.0),
            ui.y(base_y + row as f32 * 134.0),
            ui.w(328.0),
            ui.h(116.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            ui.x(560.0),
            ui.y(base_y + row as f32 * 82.0),
            ui.w(1470.0),
            ui.h(68.0),
        ),
    }
}

pub(super) fn browser_add_rect(view_mode: DeckViewMode, card_rect: Rect) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(12.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            card_rect.x + card_rect.w - ui.w(220.0),
            card_rect.y + ui.h(16.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
    }
}

pub(super) fn browser_remove_rect(view_mode: DeckViewMode, card_rect: Rect) -> Rect {
    let ui = UiLayout::current();
    match view_mode {
        DeckViewMode::Grid => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(60.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
        DeckViewMode::CompactList => Rect::new(
            card_rect.x + card_rect.w - ui.w(112.0),
            card_rect.y + ui.h(16.0),
            ui.w(92.0),
            ui.h(36.0),
        ),
    }
}

pub(super) fn deck_action_button_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(98.0 + index as f32 * 94.0),
        ui.y(214.0),
        ui.w(84.0),
        ui.h(40.0),
    )
}

pub(super) fn deck_transfer_button_rect(kind: DeckTransferActionKind) -> Rect {
    let ui = UiLayout::current();
    match kind {
        DeckTransferActionKind::Export => {
            Rect::new(ui.x(98.0), ui.y(260.0), ui.w(178.0), ui.h(40.0))
        }
        DeckTransferActionKind::Import => {
            Rect::new(ui.x(286.0), ui.y(260.0), ui.w(178.0), ui.h(40.0))
        }
    }
}

pub(super) fn deck_utility_button_rect(kind: DeckUtilityActionKind) -> Rect {
    let ui = UiLayout::current();
    match kind {
        DeckUtilityActionKind::Metadata => {
            Rect::new(ui.x(98.0), ui.y(306.0), ui.w(118.0), ui.h(34.0))
        }
        DeckUtilityActionKind::Undo => Rect::new(ui.x(223.0), ui.y(306.0), ui.w(118.0), ui.h(34.0)),
        DeckUtilityActionKind::Reset => {
            Rect::new(ui.x(348.0), ui.y(306.0), ui.w(118.0), ui.h(34.0))
        }
    }
}

pub(super) fn saved_deck_row_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(96.0),
        ui.y(364.0 + index as f32 * 60.0),
        ui.w(368.0),
        ui.h(52.0),
    )
}

pub(super) fn starter_row_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(96.0),
        ui.y(650.0 + index as f32 * 86.0),
        ui.w(222.0),
        ui.h(70.0),
    )
}

pub(super) fn starter_create_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(328.0),
        ui.y(650.0 + index as f32 * 86.0),
        ui.w(136.0),
        ui.h(70.0),
    )
}

pub(super) fn deck_builder_tab_rect(tab: DeckBuilderTab) -> Rect {
    let ui = UiLayout::current();
    match tab {
        DeckBuilderTab::SupportCards => {
            Rect::new(ui.x(560.0), ui.y(194.0), ui.w(250.0), ui.h(44.0))
        }
        DeckBuilderTab::MagicalGirls => {
            Rect::new(ui.x(826.0), ui.y(194.0), ui.w(290.0), ui.h(44.0))
        }
        DeckBuilderTab::Baddies => Rect::new(ui.x(1132.0), ui.y(194.0), ui.w(230.0), ui.h(44.0)),
    }
}

pub(super) fn roster_pool_rect(is_magical_girl_side: bool, index: usize) -> Rect {
    let ui = UiLayout::current();
    let _ = is_magical_girl_side;
    let row = index / 4;
    let column = index % 4;
    Rect::new(
        ui.x(560.0 + column as f32 * 370.0),
        ui.y(274.0 + row as f32 * 94.0),
        ui.w(348.0),
        ui.h(74.0),
    )
}

pub(super) fn summary_panel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 96.0, 390.0, 324.0)
}

pub(super) fn summary_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2110.0, 138.0, 350.0, 250.0)
}

pub(super) fn preview_panel_section_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 442.0, 390.0, 454.0)
}

pub(super) fn preview_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2120.0, 482.0, 330.0, 374.0)
}

pub(super) fn contents_panel_section_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2090.0, 918.0, 390.0, 370.0)
}

pub(super) fn contents_panel_content_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(2110.0, 958.0, 350.0, 300.0)
}

pub(super) fn roster_contents_slot_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(2110.0),
        ui.y(992.0 + index as f32 * 56.0),
        ui.w(350.0),
        ui.h(46.0),
    )
}

pub(super) fn search_input_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1100.0, 194.0, 690.0, 44.0)
}

pub(super) fn search_clear_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1806.0, 194.0, 126.0, 44.0)
}

pub(super) fn sort_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(560.0, 246.0, 156.0, 30.0)
}

pub(super) fn group_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(732.0, 246.0, 156.0, 30.0)
}

pub(super) fn view_mode_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(904.0, 246.0, 156.0, 30.0)
}

pub(super) fn filter_button_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    let buttons_per_row = 8;
    let row = index / buttons_per_row;
    let column = index % buttons_per_row;
    Rect::new(
        ui.x(1100.0 + column as f32 * 108.0),
        ui.y(246.0 + row as f32 * 38.0),
        ui.w(100.0),
        ui.h(30.0),
    )
}

pub(super) fn filter_chip_rect(index: usize) -> Rect {
    let ui = UiLayout::current();
    Rect::new(
        ui.x(1100.0 + index as f32 * 140.0),
        ui.y(324.0),
        ui.w(132.0),
        ui.h(28.0),
    )
}

pub(super) fn filter_clear_all_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1940.0, 324.0, 92.0, 28.0)
}

pub(super) fn rename_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(910.0, 520.0, 740.0, 220.0)
}

pub(super) fn rename_dialog_input_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(934.0, 574.0, 692.0, 54.0)
}

pub(super) fn rename_dialog_save_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(934.0, 650.0, 220.0, 48.0)
}

pub(super) fn rename_dialog_cancel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1170.0, 650.0, 220.0, 48.0)
}

pub(super) fn import_export_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(720.0, 360.0, 1120.0, 700.0)
}

pub(super) fn import_export_text_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(750.0, 438.0, 1060.0, 420.0)
}

pub(super) fn import_export_primary_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(750.0, 928.0, 240.0, 54.0)
}

pub(super) fn import_export_secondary_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1012.0, 928.0, 240.0, 54.0)
}

pub(super) fn import_export_close_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1570.0, 928.0, 240.0, 54.0)
}

pub(super) fn metadata_dialog_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(820.0, 360.0, 920.0, 680.0)
}

pub(super) fn metadata_tags_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 472.0, 860.0, 54.0)
}

pub(super) fn metadata_notes_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 544.0, 860.0, 340.0)
}

pub(super) fn metadata_save_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(850.0, 920.0, 220.0, 54.0)
}

pub(super) fn metadata_cancel_rect() -> Rect {
    let ui = UiLayout::current();
    ui.rect(1088.0, 920.0, 220.0, 54.0)
}
