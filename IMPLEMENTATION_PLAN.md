# Eclipse Heart Rust Implementation Plan

This document converts `raw_notes.md` into a concrete Rust build plan.

## Confirmed design direction

Eclipse Heart is a 1v1 digital collectible card game with a deck-and-roster hybrid structure.

Each player brings:

- 5 Magical Girls in a visible match roster
- 5 Baddies in a visible match roster
- 1 support deck made of story cards

Each match starts with each player choosing:

- 3 Magical Girls from their 5
- 1 Prime Baddie
- 2 Support Baddies

At setup:

- Both players reveal their 5 Magical Girls and 5 Baddies
- Both players reveal their Main Magical Girl and Prime Baddie
- The remaining chosen supports begin face-down

Win condition:

- Defeat the opposing Prime Baddie

Core identity:

- Magical Girls pressure enemy Baddies by winning Encounters
- Baddies pressure enemy Magical Girls by inflicting Strain
- The game is about balancing both fronts instead of pure unit combat

## Core match rules to implement first

### Match structure

The playable prototype should use this flow:

1. Setup
2. Daily Life
3. Encounter
4. Repeat Daily Life / Encounter
5. Final Climax
6. Resolution

Daily Life is the setup phase:

- Players alternate actions
- Players may play Daily Life cards
- Players may use Daily Life abilities
- Reaction and counter-reaction windows can happen after an action
- Daily Life ends when both players pass with no active chain

Encounter is the contest phase:

- Both players commit power through their Main Magical Girl, Prime Baddie, revealed supports, and Encounter-speed cards
- Support reveal effects mostly happen here
- Power is compared here
- Encounter results add Strain and Exposure
- Transform and Awaken checks happen here

Final Climax is the last escalation:

- It begins at the end of an Encounter once both players have a Transformed Main Magical Girl and an Awakened Prime Baddie
- Only during Final Climax can Main Magical Girls become Radiant and Prime Baddies become Catastrophe

### Resource and progression rules

Use these prototype rules from the notes:

- `Power`: simple combat number used during Encounter
- `Strain`: Magical Girl side pressure track
- `Exposure`: Baddie side pressure track
- `Bond`: persistent per-Magical-Girl resource, cap 3
- `Dread`: persistent per-Baddie resource, cap 3

Global prototype thresholds:

- Magical Girl side transforms at 6 Strain
- Baddie side awakens at 6 Exposure

Progression ladders:

- Magical Girl: `Base -> Transformed -> Radiant`
- Baddie: `Base -> Awakened -> Catastrophe`

Upgrade timing:

- Before Final Climax: Main MG can reach `Transformed`, Prime Baddie can reach `Awakened`
- During Final Climax only: Main MG can reach `Radiant`, Prime Baddie can reach `Catastrophe`

### Card classes

The implementation should support these card groups:

- Roster cards
  - Magical Girls
  - Baddies
- Main deck story cards
  - Daily Life cards
  - Reactions
  - Encounter cards
  - Bonds
  - Schemes
  - Tactics

Card speed is a first-class rule:

- `DailyLife`
- `Reaction`
- `Encounter`

## Rust architecture

The repo guidance already points to Macroquad plus `macroquad-toolkit`. Use that directly.

Recommended project structure:

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
│   │   └── ai.rs
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

### Responsibility split

- `data/`: JSON-backed definitions only
- `state/`: mutable save, match, and screen state
- `engine/`: rules resolution and validation
- `screens/`: translate UI input into intents
- `ui/`: drawing helpers and reusable widgets

UI must not apply rules directly. It should produce typed actions that the engine resolves.

## Data model

### Static content

Use JSON for all design content:

```text
assets/data/
├── ui_text.json
├── rules/
│   ├── match_rules.json
│   ├── deck_rules.json
│   └── roster_rules.json
├── magical_girls/
├── baddies/
├── story_cards/
└── starter_loadouts/
```

Recommended top-level data types:

- `MagicalGirlDefinition`
- `BaddieDefinition`
- `StoryCardDefinition`
- `MatchRules`
- `DeckRules`
- `StarterLoadout`

### Runtime match state

Keep runtime state separate from definitions:

```rust
pub enum UnitStage {
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

Recommended state structures:

- `MatchState`
- `PlayerMatchState`
- `MainMagicalGirlState`
- `SupportMagicalGirlState`
- `PrimeBaddieState`
- `SupportBaddieState`
- `EncounterState`
- `ActionStack`
- `EventLog`

Each player match state should include:

- roster reveal info
- chosen team info
- support deck / hand / discard
- current Strain
- current Exposure
- per-unit Bond or Dread counters
- revealed or hidden support slots

### Actions and events

Use explicit action and event enums.

Core action families:

- roster selection
- reveal support
- play card
- use unit ability
- pass priority
- commit to Encounter
- resolve transformation
- resolve awakening
- trigger Final Climax choice

Core event families:

- card played
- support revealed
- Strain changed
- Exposure changed
- Bond changed
- Dread changed
- unit exhausted
- unit transformed
- unit awakened
- unit became Radiant
- unit became Catastrophe
- Prime Baddie defeated

This event log should drive battle UI, debugging, and tests.

## Engine design

### Deterministic resolution

The rules engine should be deterministic and UI-agnostic.

Use an engine entry like:

```rust
pub fn apply_action(
    state: &mut MatchState,
    rules: &MatchRules,
    action: PlayerAction,
) -> ActionResolution
```

Resolution order should be:

1. Validate legal timing
2. Validate card targets
3. Apply costs and exhaustion
4. Apply primary effect
5. Apply triggered reactions
6. Update Strain / Exposure / Bond / Dread
7. Check transform or awaken thresholds
8. Check Final Climax trigger
9. Check Prime Baddie defeat
10. Emit events

### Encounter resolution

Keep Encounter resolution simple for the first playable:

1. Determine active modifiers
2. Sum MG-side Encounter power
3. Sum Baddie-side Encounter pressure
4. Compare results
5. Assign standard rewards

Prototype reward rule from the notes:

- winner side gains 1 pressure on itself
- loser side gains 2 pressure on the opposing front

Translated to current terms:

- if your MG side wins an Encounter, your MG side gains 1 Strain and the enemy Baddie side gains 2 Exposure
- if your Baddie pressure wins, your Baddie side gains 1 Exposure and the enemy MG side gains 2 Strain

This should live in rules JSON so the numbers can be tuned without code edits.

### Progression system

Progression must be generic, because any of the five rostered units can be chosen.

Each Magical Girl definition needs:

- base form stats and text
- transformed form stats and text
- radiant form stats and text

Each Baddie definition needs:

- base form stats and text
- awakened form stats and text
- catastrophe form stats and text

Use these implementation rules:

- base form carries identity
- transformed / awakened adds one tactical once-per-Encounter ability
- radiant / catastrophe adds one once-per-Final-Climax payoff ability

## Save and collection model

The game goal is collectible and tradeable, so save structure matters early.

MVP local save should include:

- owned Magical Girls
- owned Baddies
- owned story cards
- saved support decks
- saved 5-card roster presets
- settings
- profile progression

Do not mix card definitions with player ownership.

Recommended save files:

- `save/profile.json`
- `save/collection.json`
- `save/decks.json`
- `save/settings.json`

Version all save data from day one.

## Online and trading plan

The notes describe an online competitive game, but the first Rust implementation should not block on backend work.

Recommended delivery order:

1. Offline playable client
2. Local collection and deck persistence
3. AI opponent
4. Local simulated trade model
5. Separate Rust service for account, ownership, and trade authority
6. Online PvP authority

When online work starts, keep it in a separate crate or service. Do not push networking into the Macroquad client core.

## First playable milestone

The first playable should prove the game loop, not content scale.

Target scope:

- 1 playable battle screen
- 5 Magical Girls
- 5 Baddies
- 20 story cards from the current notes
- 1 starter support deck per side
- roster selection from 5 into chosen 3
- hidden support reveal
- Strain / Exposure / Bond / Dread tracking
- Base -> Transformed and Base -> Awakened progression
- Final Climax trigger
- Yuki and Noctra full upgraded ladder support

Delay these until after the loop works:

- full card animations
- bundle shop
- boosters
- fusions
- online trading
- ranked multiplayer

## Suggested implementation phases

### Phase 1: Crate and data scaffolding

- create `eclipse_heart` Cargo crate
- add Macroquad dependencies
- add `assets/data/` structure
- define Serde schemas for cards and rules

### Phase 2: Match engine skeleton

- define `MatchState`
- define `PlayerAction`
- define `MatchEvent`
- implement Setup, Daily Life, Encounter, Final Climax phases

### Phase 3: Core prototype rules

- implement pressure tracks
- implement reveal rules
- implement exhaustion
- implement transform and awaken checks
- implement win condition

### Phase 4: Content slice

- load 20 story cards from JSON
- load 5 Magical Girls and 5 Baddies
- encode Yuki and Noctra full form ladders

### Phase 5: UI shell

- main menu
- deck/roster builder
- match setup
- battle screen
- result screen

### Phase 6: Persistence and tests

- save collection and decks
- add loader tests
- add encounter resolution tests
- add progression threshold tests

## Open design gaps from the notes

These points still need a final call before implementation gets too far:

- exact support deck size: the notes mention `20`, `30`, and `30 to 40`
- whether support Baddies can ever progress past base by default
- whether support Magical Girls can transform outside rare effects
- exact defeat rule for Prime Baddies
- exact opening hand size and mulligan rules
- whether cards can target hidden support slots directly or only force reveals

Until those are finalized, code the engine so these values come from rules/config instead of hardcoding them.
