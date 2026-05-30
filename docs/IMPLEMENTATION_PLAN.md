# Eclipse Heart Rust Implementation Plan

This document converts `RULES.md` and `raw_notes.md` into a concrete Rust implementation plan.

## 1. Confirmed game shape

Eclipse Heart is a 1v1 digital collectible card game with:

- a Magical Girl roster
- a Baddie roster
- a support deck of story cards

Each player brings:

- `5` Magical Girls
- `5` Baddies
- `1` support deck of `40` cards

Each match uses:

- per player:
  - `1` Main Magical Girl
  - `2` Support Magical Girls
  - `1` Prime Baddie
  - `2` Support Baddies

Win condition:

- defeat the opposing `Prime Baddie` during `Final Climax`

## 2. Core implementation goals

The first Rust implementation should prove these systems:

1. roster reveal and hidden support selection
2. Daily Life and Encounter turn flow
3. reaction chains and reveal timing
4. per-character `Radiance / Dread` growth
5. `Base -> Transformed -> Radiant`
6. `Base -> Awakened -> Catastrophe`
7. exhaustion
8. Final Climax declaration
9. Prime Baddie defeat resolution

## 3. Rust architecture

Use the repository standard stack:

- Rust 2021
- Macroquad
- `macroquad-toolkit`
- `serde`
- `serde_json`

Recommended structure:

```text
eclipse_heart/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── game.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── cards.rs
│   │   ├── rosters.rs
│   │   ├── rules.rs
│   │   └── loader.rs
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── match_engine.rs
│   │   ├── action_validator.rs
│   │   ├── encounter.rs
│   │   ├── progression.rs
│   │   ├── timing.rs
│   │   └── final_climax.rs
│   ├── state/
│   │   ├── mod.rs
│   │   ├── app_state.rs
│   │   ├── profile.rs
│   │   ├── collection.rs
│   │   ├── decks.rs
│   │   ├── match_state.rs
│   │   └── persistence.rs
│   ├── screens/
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── roster_select.rs
│   │   ├── collection.rs
│   │   ├── deck_builder.rs
│   │   ├── battle.rs
│   │   └── results.rs
│   └── ui/
│       ├── mod.rs
│       ├── core.rs
│       ├── card_widgets.rs
│       └── layout.rs
└── assets/
    └── data/
```

Responsibility split:

- `data/`: JSON-backed definitions only
- `state/`: mutable runtime and save state
- `engine/`: legal actions, resolution, and rule enforcement
- `screens/`: UI intent capture
- `ui/`: drawing helpers

UI must never apply rules directly.
UI should be fully playable with the mouse alone; keyboard input may be added later as optional convenience, not as a requirement.

## 4. Data model

### Static definitions

Use JSON for all design content:

```text
assets/data/
├── ui_text.json
├── rules/
│   ├── match_rules.json
│   ├── deck_rules.json
│   └── progression_rules.json
├── magical_girls/
├── baddies/
├── story_cards/
└── starter_loadouts/
```

Core definition types:

- `MagicalGirlDefinition`
- `BaddieDefinition`
- `StoryCardDefinition`
- `MatchRules`
- `StarterLoadout`

Each character definition needs:

- character id
- display name key
- base Power
- transformed/awakened Power
- radiant/catastrophe Power
- first threshold
- second threshold
- reveal ability text/effects
- transformed ability/effects
- final-form ability/effects

### Runtime state

Keep runtime state separate from static definitions.

Suggested core enums:

```rust
pub enum CharacterStage {
    Base,
    Transformed,
    Radiant,
    Awakened,
    Catastrophe,
}

pub enum MatchPhase {
    Setup,
    DailyLife,
    Encounter,
    FinalClimax,
    Finished,
}
```

Suggested runtime structures:

- `MatchState`
- `PlayerMatchState`
- `CharacterRuntimeState`
- `SupportSlotState`
- `EncounterState`
- `ReactionStack`
- `EventLog`

Each `CharacterRuntimeState` should contain:

- definition id
- current stage
- current Power modifiers
- current `Radiance` or `Dread`
- exhausted flag
- revealed flag if support
- threshold values copied from definition or referenced from definition

Each player match state should contain:

- `player_id`
- full 5 MG roster
- full 5 Baddie roster
- selected Main MG
- selected Prime Baddie
- hidden or revealed support slots for both rosters
- deck / hand / discard

`MatchState` should additionally track:

- `player_a`
- `player_b`
- `active_player`
- current engagement lane
  - active player's Magical Girls
  - versus opposing player's Baddies
- reaction stack / priority state

## 5. Core rules model

### Growth stats

The old `Bond / Strain / Exposure` model is obsolete.

Use only:

- `Radiance` for Magical Girls
- `Dread` for Baddies

Rules:

- growth is per-character
- values never go below zero
- thresholds vary by character
- upgrade progress resets when a threshold is reached
- overflow is lost
- upgrades do not revert unless a card says so

### Progression

Magical Girls:

- `Base -> Transformed -> Radiant`

Baddies:

- `Base -> Awakened -> Catastrophe`

All units, including supports, can reach final form.

Upgrades happen immediately when:

1. an effect resolves
2. the resulting `Radiance` or `Dread` meets threshold

Reactions can still stop the gain before resolution.

### Encounter rewards

Default reward mapping:

- winner gains `+3`
- loser gains `+1`
- tie gives both `+2`

Apply rewards to:

- Magical Girls as `Radiance`
- Baddies as `Dread`

Implementation note:

- reward functions should still live in config so tuning remains data-driven

### Power calculation

Encounter Power is:

- the active player's Main MG Power plus all revealed, non-exhausted Support MG Power
- versus the opposing player's Prime Baddie Power plus all revealed, non-exhausted Support Baddie Power

Hidden supports add nothing.
Exhausted units add no Power and cannot use abilities.

## 6. Timing engine

The timing system needs to support:

- Daily Life actions
- Encounter actions
- reactions
- reactions to reactions
- support reveals as either actions or reactions
- newest-to-oldest trigger resolution

Recommended approach:

- resolve player actions through a typed command enum
- use a reaction stack
- emit events as each step resolves

Core action families:

- choose Main MG / Prime Baddie for each player
- choose hidden supports for each player
- reveal support
- play card
- use character ability
- pass
- declare Final Climax

Core event families:

- card played
- support revealed
- radiance changed
- dread changed
- unit exhausted
- unit stage changed
- Final Climax declared
- Prime Baddie defeated

## 7. Final Climax implementation

Current rule:

- Final Climax may be declared only at the start of an Encounter
- only the active player's Main MG may declare it
- that Main MG must be fully transformed
- the opposing player cannot stop the declaration unless a card explicitly says so

Prime defeat rule:

- if it is Final Climax and the defending player's Prime Baddie's total Power is lower when the Encounter ends, it is defeated

Tie rule:

- no defeat on tie
- ties may still grant growth points
- fully transformed characters cannot use tie points for further growth

Loss penalty:

- if the active player's Main MG loses Final Climax, it becomes exhausted for one turn
- it does not count its Power during the next Encounter

Implementation recommendation:

- track `final_climax_declared: bool`
- track which player is attacking and which player is defending during the current Encounter
- reuse normal Encounter resolution with a Final Climax flag
- attach post-resolution defeat checks only when that flag is active

## 8. Hidden support handling

Setup rules require real hidden information.

Implementation rules:

- each player sees the opponent's original 5 MG and 5 Baddie rosters
- each player does not know which 2 support units were chosen on either opposing roster
- hidden support slots should store only owning-player-visible identity
- hidden supports cannot be targeted unless a card explicitly reveals or affects hidden cards

UI implications:

- owning player sees exact hidden support identity for both of their rosters
- opponent sees only hidden slots

## 9. Exhaustion

Exhaustion blocks:

- Power contribution
- abilities

Default timing:

- exhausted during Daily Life -> no Power in the next Encounter
- exhausted mid-Encounter -> no Power for that Encounter
- exhausted at end of Encounter -> remains exhausted through end of next Encounter

Implementation recommendation:

- track an exhaustion expiry marker instead of a simple bool if needed
- example: `UntilCurrentEncounterEnds`, `UntilNextEncounterEnds`

## 10. Save and collection model

MVP local save should contain:

- owned Magical Girls
- owned Baddies
- owned story cards
- saved support decks
- roster presets
- settings

Character ownership is separate from card definitions.

Suggested save files:

- `save/profile.json`
- `save/collection.json`
- `save/decks.json`
- `save/settings.json`

Version save data from the start.

## 11. First playable content slice

Use the currently designed prototype set:

- `20` story cards
- `5` Magical Girls
- `5` Baddies

First milestone content target:

- encode the 20 current story cards as JSON
- encode 5 Magical Girls
- encode 5 Baddies
- fully implement Yuki and Noctra as the first progression templates

## 12. Suggested implementation phases

### Phase 1: crate and data scaffolding

- create `eclipse_heart` Cargo crate
- add workspace entry
- add dependencies
- scaffold `src/` tree
- scaffold `assets/data/`

### Phase 2: runtime state and loaders

- implement JSON schemas
- implement loaders
- implement runtime state structs
- implement save data structs

### Phase 3: timing and action engine

- implement action enums
- implement reaction stack
- implement trigger ordering
- implement reveal rules

### Phase 4: progression and Encounter resolution

- implement growth gain and reduction
- implement threshold checks
- implement upgrade resets
- implement Power summing
- implement exhaustion

### Phase 5: Final Climax and win condition

- implement declaration rules
- implement Final Climax Encounter flag
- implement Prime defeat check
- implement tie handling

### Phase 6: UI shell

- main menu
- deck builder
- roster selection
- battle screen
- result screen
- all core flows must be mouse-driven so a player can navigate and complete a match without touching the keyboard

### Phase 7: persistence and tests

- save/load
- loader tests
- progression tests
- Encounter resolution tests
- Final Climax tests

## 13. Testing priorities

Focus tests on:

- JSON loading
- hidden support secrecy
- reaction ordering
- threshold crossing and upgrade reset
- exhaustion duration
- Final Climax declaration legality
- Prime Baddie defeat checks

## 14. Remaining design edge cases

Core rules are settled, but implementation may still need exact wording for:

- simultaneous downgrade and upgrade interactions
- rare card-specific reveal exceptions
- rare card-specific exhaustion exceptions
- simultaneous replacement effects during Final Climax
