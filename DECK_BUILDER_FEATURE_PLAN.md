# Deck Builder Feature Plan

## Goal

Expand the current prototype deck editor into a full-featured deck builder that matches the usability expectations of modern online card games while fitting `Eclipse Heart`'s rules model:

- support-card deck editing
- separate Magical Girl roster editing
- separate Baddie roster editing
- ownership-aware collection browsing
- template-based deck creation
- deck validation, import/export, and missing-card handling

This plan treats the current implementation as a prototype base, not a finished builder.

## Product Principles

- Keep support-card editing separate from roster editing.
- Keep saved deck data distinct from transient UI/search/filter state.
- Always show whether a deck is legal, complete, and playable.
- Make collection browsing fast: search, filter, sort, and preview should be first-class.
- Preserve edits automatically and avoid destructive starter/template reload behavior.
- Keep the builder mouse-driven and readable at the `2560 x 1440` baseline.

## Current State

Already implemented:

- persistent support deck presets
- persistent per-starter editing
- separate roster-edit layer
- owned-copy-aware support-card editing
- starter/template entry points
- booster acquisition from the global pool
- card preview panel

Current limitations:

- no real deck list manager
- no search or filter system
- no sort/group/view options
- no legality summary
- no deck import/export
- no missing-card visualization or replacement flow
- no explicit template creation flow beyond starter editing
- no deck metadata beyond id/name/cards

## Target Builder Structure

The finished builder should have five major pieces:

1. `Deck List`
- saved decks
- templates/starters
- create, duplicate, rename, delete

2. `Deck Summary`
- deck name
- legal/illegal state
- support-card count
- Magical Girl roster status
- Baddie roster status
- missing-card count

3. `Collection Browser`
- search
- filters
- sort
- grouping
- result count

4. `Deck Editor`
- support-card deck contents
- add/remove interactions
- per-card count visibility

5. `Roster Editor`
- 5 Magical Girl slots
- 5 Baddie slots
- swap-based editing with no duplicates

## Data Model Plan

### 1. Expand `DeckPreset`

Current direction in [src/state/decks.rs](/H:/RustGames/eclipse_heart/src/state/decks.rs) is good, but the final model should include:

```rust
pub struct DeckPreset {
    pub id: String,
    pub name: String,
    pub source_template_id: Option<String>,
    pub story_cards: Vec<String>,
    pub magical_girl_roster: Vec<String>,
    pub baddie_roster: Vec<String>,
    pub notes: String,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}
```

Recommended additions:

- `source_template_id`
- `notes`
- `created_at_unix`
- `updated_at_unix`

### 2. Add Deck Validation Model

Add a non-persistent validation struct, likely under `src/state/` or a future `src/deck_builder/` module:

```rust
pub struct DeckValidation {
    pub support_card_count: usize,
    pub required_support_card_count: usize,
    pub support_card_count_valid: bool,
    pub magical_girl_roster_valid: bool,
    pub baddie_roster_valid: bool,
    pub duplicate_story_cards: Vec<String>,
    pub missing_story_cards: Vec<String>,
    pub missing_magical_girls: Vec<String>,
    pub missing_baddies: Vec<String>,
    pub is_collection_complete: bool,
    pub is_legal: bool,
}
```

### 3. Add Builder UI State

This should not live in saves. It belongs in runtime UI state, likely in [src/state/app_state.rs](/H:/RustGames/eclipse_heart/src/state/app_state.rs) or a new builder-specific UI module:

```rust
pub struct DeckBuilderUiState {
    pub selected_deck_id: String,
    pub active_tab: DeckBuilderTab,
    pub search_text: String,
    pub filters: DeckBuilderFilters,
    pub sort_mode: DeckSortMode,
    pub group_mode: DeckGroupMode,
    pub view_mode: DeckViewMode,
    pub selected_preview_card_id: Option<String>,
    pub selected_mg_slot: Option<usize>,
    pub selected_baddie_slot: Option<usize>,
}
```

## Feature Phases

## Phase 1: Core Deck Management

### Scope

- Replace the current “active deck only” mental model with explicit selected deck management.
- Add real deck list controls.
- Stop using starters as the only visible source of decks.

### Features

- create new deck
- rename deck
- duplicate deck
- delete deck
- select deck from a deck list
- mark starter/template origin separately from editable deck records

### Acceptance Criteria

- user can create multiple saved decks
- user can switch between decks without losing edits
- user can duplicate a deck and edit the copy independently
- user can delete a deck without affecting templates

## Phase 2: Deck Legality and Completion Feedback

### Scope

- Add a permanent legality/validation summary panel.

### Features

- support-card count summary
- MG roster count summary
- Baddie roster count summary
- duplicate warnings
- missing-card warnings
- overall legal/playable status

### Acceptance Criteria

- every selected deck visibly shows whether it is legal
- invalid states are readable without opening extra panels
- support-card and roster issues are reported separately

## Phase 3: Search

### Scope

- Add a text search bar over the collection browser.

### Requirements

- partial text matching
- multiple terms
- search across:
  - card name
  - card text
  - card type
  - speed
  - alignment
  - ownership state

### Extended Query Syntax

Initial recommended supported tags:

- `speed:daily`
- `speed:reaction`
- `speed:encounter`
- `align:mg`
- `align:baddie`
- `align:neutral`
- `type:<value>`
- `owned`
- `missing`
- `copies:<n>`

### Acceptance Criteria

- text search updates visible results
- search combines with filters
- invalid query terms fail softly

## Phase 4: Filters

### Scope

- Add faceted filtering similar to online card libraries.

### Initial Filter Set For `Eclipse Heart`

- speed
- alignment
- card type
- owned state
- missing state
- in current deck
- not in current deck

### Later Filter Extensions

- effect tags
- growth/power/reveal/exhaust tags
- set
- rarity, if added later

### Acceptance Criteria

- multiple filters can be active together
- active filters are visible as chips/tags
- each active filter can be removed individually
- clear-all is available

## Phase 5: Sort, Group, and View Modes

### Sort

Minimum:

- alphabetical
- newest
- owned count
- copies in deck

### Group

Minimum:

- none
- by alignment
- by speed
- by card type

### View

Minimum:

- grid view
- compact list view

### Acceptance Criteria

- sort applies after filtering
- grouping remains readable in both views
- current sort/group/view state is always visible

## Phase 6: Proper Deck Editor Layout

### Recommended Layout

- left rail: deck list and templates
- upper center: deck summary and legality
- main center: collection browser
- right rail: preview + current deck contents
- tabs: `Support Cards`, `Magical Girl Roster`, `Baddie Roster`

### Interaction Rules

- support cards:
  - add/remove directly
  - show owned count, deck count, max copies
- rosters:
  - swap into fixed 5 slots
  - prevent duplicates by swap behavior
  - always preserve exactly 5 cards

### Acceptance Criteria

- collection browsing and deck contents are visible at the same time
- users do not need to jump between unrelated screens to understand deck state
- roster editing stays separate from support-card editing

## Phase 7: Templates and Deck Recipes

### Scope

- Turn starter loadouts into proper deck templates.

### Features

- `Create Deck From Template`
- show template description
- show template playstyle
- show template roster + support-card seed
- create a new editable deck from a template instead of editing the template directly

### Acceptance Criteria

- starter/template data remains reusable
- templates do not overwrite user decks
- creating from template is one obvious action

## Phase 8: Import / Export / Sharing

### Scope

- Add a portable deck code format.

### Features

- export current deck to string
- import deck from string
- copy to clipboard
- paste/import from clipboard

### Rules

- imports should reconstruct the deck even if the user lacks some cards
- imported decks should render even when incomplete

### Acceptance Criteria

- export then import round-trips deck contents correctly
- incomplete imports remain visible and editable

## Phase 9: Missing Cards and Replacement Suggestions

### Scope

- Add ownership-aware incomplete deck handling.

### Features

- missing cards shown with distinct visual styling
- missing copies counted in validation summary
- ownership filters
- suggested replacements

### Suggested Replacement Rules

Initial heuristic:

1. same speed
2. same alignment
3. same card type
4. owned by player

### Acceptance Criteria

- imported/template decks clearly show what is missing
- missing cards do not break rendering
- replacement suggestions never violate copy limits or duplicate roster rules

## Phase 10: Collection and Meta Polish

### Features

- recent cards
- recently modified decks
- deck notes
- deck archetype tags
- deck thumbnails or faction banners
- undo last change
- reset deck to template

### Acceptance Criteria

- builder supports longer-term collection management, not just one-session editing

## UI Recommendations Based On Popular Online Card Games

Adapt these patterns rather than copying them literally:

### Search Bar

- top-center or top-left above the collection browser
- instant update as query changes
- supports lightweight query tags

### Active Filter Chips

- horizontal row under the search bar
- removable individually
- includes clear-all action

### Result Count

- show total result count near search/filter controls
- optionally show deck count alongside it

### Card Evaluation

- collection tiles should show enough to assess a card quickly:
  - name
  - speed
  - alignment
  - type
  - short rule text or keyword summary
  - owned copies
  - copies in deck

### Missing Card State

- visibly muted tile
- still previewable
- still searchable/filterable

### Deck Summary Rail

- should always show:
  - deck name
  - legal/illegal
  - support count
  - MG roster count
  - Baddie roster count
  - missing count

## Technical Implementation Order

Recommended engineering order:

1. deck list manager and selected-deck state
2. validation summary helpers
3. search/filter/sort data model
4. split collection browser from deck contents panel
5. templates/create-from-template flow
6. import/export deck code support
7. missing-card handling and replacements
8. polish: grouping, view modes, notes, reset, undo

## Proposed Module Structure

Longer-term, the deck builder likely deserves its own module cluster:

- `src/deck_builder/mod.rs`
- `src/deck_builder/validation.rs`
- `src/deck_builder/query.rs`
- `src/deck_builder/import_export.rs`
- `src/deck_builder/replacements.rs`
- `src/deck_builder/ui_state.rs`

Screen code in [src/screens/deck_builder.rs](/H:/RustGames/eclipse_heart/src/screens/deck_builder.rs) can then become a thin controller/view layer instead of carrying all logic directly.

## Testing Plan

Add focused tests for:

- creating/selecting/deleting/duplicating decks
- deck validation legality summary
- roster swap behavior
- search matching
- filter combinations
- import/export round-trip
- missing-card detection
- replacement suggestions
- template-to-new-deck creation

## Definition of Done For “Full Builder”

The deck builder should be considered feature-complete when:

- multiple user decks can be managed cleanly
- templates/starters create decks without overwriting user edits
- support cards and rosters can both be edited intuitively
- legality is always visible
- search/filter/sort make collection browsing fast
- import/export works
- missing-card decks are still renderable and understandable

## Immediate Next Milestone

The next best concrete milestone is:

1. add real deck list management
2. add legality summary
3. add search + filter bar

That combination will move the builder from “prototype editor” to “credible deck builder” faster than any other slice.
