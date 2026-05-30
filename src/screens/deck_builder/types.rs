use macroquad::prelude::Rect;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum DeckBuilderTab {
    SupportCards,
    MagicalGirls,
    Baddies,
}

pub(super) struct DeckRenameDialog {
    pub(super) value: String,
}

impl DeckRenameDialog {
    pub(super) fn new(current_name: &str) -> Self {
        Self {
            value: current_name.to_owned(),
        }
    }
}

pub(super) struct DeckImportExportDialog {
    pub(super) mode: DeckImportExportMode,
    pub(super) value: String,
    pub(super) status: Option<String>,
    pub(super) text_focused: bool,
}

impl DeckImportExportDialog {
    pub(super) fn for_export(value: String) -> Self {
        Self {
            mode: DeckImportExportMode::Export,
            value,
            status: None,
            text_focused: false,
        }
    }

    pub(super) fn for_import() -> Self {
        Self {
            mode: DeckImportExportMode::Import,
            value: String::new(),
            status: None,
            text_focused: true,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum DeckImportExportMode {
    Export,
    Import,
}

pub(super) struct DeckMetadataDialog {
    pub(super) notes: String,
    pub(super) tags: String,
    pub(super) notes_focused: bool,
    pub(super) tags_focused: bool,
}

impl DeckMetadataDialog {
    pub(super) fn new(notes: &str, tags: &str) -> Self {
        Self {
            notes: notes.to_owned(),
            tags: tags.to_owned(),
            notes_focused: true,
            tags_focused: false,
        }
    }
}

pub(super) struct DeckActionButton<'a> {
    pub(super) kind: DeckActionKind,
    pub(super) label: &'a str,
    pub(super) enabled: bool,
}

pub(super) struct DeckTransferButton<'a> {
    pub(super) kind: DeckTransferActionKind,
    pub(super) label: &'a str,
    pub(super) enabled: bool,
}

pub(super) struct DeckUtilityButton<'a> {
    pub(super) kind: DeckUtilityActionKind,
    pub(super) label: &'a str,
    pub(super) enabled: bool,
}

pub(super) struct FilterButton {
    pub(super) kind: FilterButtonKind,
    pub(super) label: String,
    pub(super) active: bool,
}

impl FilterButton {
    pub(super) fn new(label: impl Into<String>, kind: FilterButtonKind, active: bool) -> Self {
        Self {
            kind,
            label: label.into(),
            active,
        }
    }
}

pub(super) struct FilterChip {
    pub(super) kind: FilterChipKind,
    pub(super) label: String,
}

impl FilterChip {
    pub(super) fn new(label: impl Into<String>, kind: FilterChipKind) -> Self {
        Self {
            kind,
            label: label.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum DeckActionKind {
    Create,
    Rename,
    Duplicate,
    Delete,
}

#[derive(Clone, Copy)]
pub(super) enum DeckTransferActionKind {
    Export,
    Import,
}

#[derive(Clone, Copy)]
pub(super) enum DeckUtilityActionKind {
    Metadata,
    Undo,
    Reset,
}

#[derive(Clone)]
pub(super) enum FilterButtonKind {
    Speed(crate::data::CardSpeed),
    Alignment(crate::data::CardAlignment),
    CardType(String),
    OwnedOnly,
    MissingOnly,
    InDeckOnly,
    NotInDeckOnly,
}

#[derive(Clone)]
pub(super) enum FilterChipKind {
    Speed(crate::data::CardSpeed),
    Alignment(crate::data::CardAlignment),
    CardType(String),
    OwnedOnly,
    MissingOnly,
    InDeckOnly,
    NotInDeckOnly,
}

pub(super) enum BrowserLayoutItem<'a> {
    GroupHeader { label: String, rect: Rect },
    Card(BrowserCardLayout<'a>),
}

pub(super) struct BrowserCardLayout<'a> {
    pub(super) card: &'a crate::data::StoryCardDefinition,
    pub(super) rect: Rect,
    pub(super) add_rect: Rect,
    pub(super) remove_rect: Rect,
}
