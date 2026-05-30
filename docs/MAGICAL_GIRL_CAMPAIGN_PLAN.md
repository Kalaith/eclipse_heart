# Magical Girl Campaign Production Plan

## Goal

Move `Eclipse Heart` from a battle prototype into a production-ready single-player game where the player takes a Magical Girl through a full AI-driven campaign and wins a sequence of escalating battles.

This plan assumes the deck builder is now a credible production foundation. The next milestone is no longer “make one match work.” It is “make a full player-facing game loop work.”

## Product Vision

The production game should feel like:

- a single-player Magical Girl campaign
- a sequence of curated or semi-random AI battles
- persistent progression between battles
- deck and roster decisions that matter over a run
- clear failure, recovery, and victory states
- a clean player loop:
  - choose or continue a campaign
  - prepare
  - fight an AI battle
  - collect rewards
  - upgrade deck and roster
  - advance to the next battle
  - defeat the final opponent and win the campaign

The player fantasy is:

- “I am building and piloting a Magical Girl run”
- not “I am configuring both sides of a prototype skirmish”

## Core Product Principles

- The player controls one campaign side only.
- The player should always be the Magical Girl side in campaign mode.
- The opponent is always AI-controlled and should be presented as a named enemy encounter, not as another setup screen participant.
- Campaign progression should be persistent and resumable.
- Preparation, rewards, and failure recovery should be explicit parts of the loop.
- The deck builder should remain the long-term collection management tool, while campaign prep should focus on run decisions.
- Data for encounters, rewards, chapters, and campaign structure should be content-driven where practical.

## Current State

Already implemented:

- battle rules engine
- AI turn selection for battles
- match setup for both sides
- saved support decks and rosters
- deck builder with templates, import/export, notes, tags, undo, reset, and validation
- collection persistence
- booster opening from the global pool
- simulation tooling for matchup testing

Current production gaps:

- no campaign mode
- no player-only flow
- no encounter map or chapter progression
- no run state
- no battle rewards loop
- no campaign-specific prep screen
- no loss/retry/continue campaign structure
- no final campaign victory flow
- no enemy encounter catalog or boss ladder
- no progression economy beyond generic owned cards
- no run-scoped upgrades, healing, or branching choices
- no UI shell for campaign summary, encounter selection, or post-battle rewards

## Target Player Loop

The production campaign loop should be:

1. Start or continue campaign
2. Choose campaign save or start a fresh run
3. Select initial Magical Girl identity and opening deck
4. Enter campaign map or chapter screen
5. Pick next encounter
6. Fight AI-controlled enemy battle
7. Resolve victory or defeat
8. If victorious:
   - receive rewards
   - modify deck or roster
   - advance campaign state
9. If defeated:
   - fail campaign or use limited recovery rules
10. Reach final encounter and win campaign

## Campaign Structure Recommendation

Recommended first production structure:

- `1` player-controlled Magical Girl campaign run
- `3` acts
- `3` to `5` encounters per act
- `1` named boss encounter at the end of each act
- `1` final campaign boss at the end of the run

This is enough to create a real progression arc without immediately requiring a giant content set.

## Game Modes Recommendation

To reduce delivery risk, split modes clearly:

### 1. `Skirmish`

Current setup-driven battle mode, useful for:

- sandbox play
- debugging
- balance checks

### 2. `Campaign`

The production-facing single-player mode:

- player controls Magical Girl side
- AI controls enemy side
- persistent run state
- rewards and progression

Skirmish can remain for internal utility and optional player experimentation, but campaign should become the default mainline experience.

## Target Campaign Systems

The production campaign should have these major systems:

1. `Campaign Save Slots`
- new run
- continue run
- abandon run
- campaign completion record

2. `Campaign State`
- current act
- current node or encounter
- victories and losses
- rewards taken
- current deck snapshot
- current roster snapshot
- health or run condition if used

3. `Encounter Catalog`
- enemy identity
- AI deck
- AI roster
- reward table
- intro text
- boss flags

4. `Campaign Map or Chapter Flow`
- linear or lightly branching encounters
- current position
- completed nodes
- next available nodes

5. `Pre-Battle Preparation`
- view current deck
- view current roster
- optionally swap to another saved deck or campaign deck
- read encounter details

6. `Battle Rewards`
- booster-like card gain
- targeted draft choice
- currency
- healing or upgrade effects
- roster unlocks, if added later

7. `Post-Battle Resolution`
- victory rewards
- defeat handling
- campaign advancement

8. `Campaign Victory`
- final boss defeated
- campaign clear summary
- unlock or meta progression hooks

## Data Model Plan

### 1. Add Campaign Save Data

Recommended new persistent file:

- `save/campaigns.json`

Suggested top-level direction:

```rust
pub struct CampaignSaveBundle {
    pub version: u32,
    pub slots: Vec<CampaignRunSave>,
    pub completed_runs: Vec<CampaignCompletionRecord>,
}
```

### 2. Add Campaign Run Save

Suggested direction:

```rust
pub struct CampaignRunSave {
    pub id: String,
    pub name: String,
    pub campaign_id: String,
    pub status: CampaignRunStatus,
    pub current_act_index: usize,
    pub current_node_id: String,
    pub completed_node_ids: Vec<String>,
    pub player_deck: DeckPreset,
    pub rewards_taken: Vec<String>,
    pub battle_history: Vec<CampaignBattleRecord>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}
```

Important principle:

- campaign run deck state should be a snapshot or fork of a saved deck at run start
- campaign edits during the run should not silently overwrite the player’s collection deck unless that is an explicit player action

### 3. Add Campaign Content Definitions

Recommended new data folder:

- `assets/data/campaigns/`

Suggested content types:

- `campaign_definition.json`
- `encounters/*.json`
- `reward_tables.json`

Example direction:

```rust
pub struct CampaignDefinition {
    pub id: String,
    pub name: String,
    pub acts: Vec<CampaignActDefinition>,
}

pub struct CampaignActDefinition {
    pub id: String,
    pub name: String,
    pub nodes: Vec<CampaignNodeDefinition>,
}

pub struct CampaignNodeDefinition {
    pub id: String,
    pub encounter_id: String,
    pub next_node_ids: Vec<String>,
    pub boss: bool,
}

pub struct EncounterDefinition {
    pub id: String,
    pub name: String,
    pub ai_template_id: String,
    pub reward_table_id: String,
    pub intro_text: String,
    pub boss: bool,
}
```

### 4. Add Battle Entry Context

Skirmish and campaign should not share the exact same entry assumptions.

Recommended direction:

```rust
pub enum BattleContext {
    Skirmish,
    Campaign {
        run_id: String,
        node_id: String,
        encounter_id: String,
    },
}
```

That lets post-battle resolution route correctly.

## Recommended Feature Phases

## Phase 1: Split Skirmish From Campaign

### Scope

- Stop treating the current setup screen as the primary game loop.
- Introduce a clear mode split.

### Features

- main menu shows `Campaign` and `Skirmish`
- skirmish keeps current prototype setup flow
- campaign gets its own placeholder screen

### Acceptance Criteria

- player can choose campaign without seeing the dual-player prototype setup
- skirmish still works unchanged

## Phase 2: Campaign Save Slots

### Scope

- Add persistent campaign run slots.

### Features

- new run
- continue run
- abandon run
- campaign slot summary

### Acceptance Criteria

- player can exit and resume a campaign
- run status persists correctly

## Phase 3: Encounter Content and AI Opponents

### Scope

- Add content definitions for campaign encounters.

### Features

- named AI opponents
- enemy deck source
- enemy roster source
- boss flags
- encounter intro text

### Acceptance Criteria

- campaign battles no longer need manual enemy setup
- encounters are data-driven

## Phase 4: Campaign Map or Chapter Progression

### Scope

- Add campaign advancement between battles.

### Recommended First Version

- linear acts with a small amount of branching, not a full roguelike map

### Features

- current node
- completed nodes
- next encounter selection
- act transition

### Acceptance Criteria

- player can advance through a full run structure
- completed encounters do not replay accidentally unless designed to

## Phase 5: Campaign Battle Entry

### Scope

- Enter battles directly from campaign state.

### Features

- campaign node launches battle
- player uses campaign run deck snapshot
- AI uses encounter-defined deck and roster
- post-battle returns to campaign flow

### Acceptance Criteria

- player no longer configures both sides manually for campaign
- campaign battle setup is automatic and correct

## Phase 6: Post-Battle Results and Rewards

### Scope

- Make victory matter and power the run forward.

### Features

- victory screen
- defeat screen
- reward selection
- apply reward to campaign run state

### Recommended First Reward Set

- choose `1` of `3` support cards
- open a themed booster
- small heal or recovery rule if health exists
- currency or unlock token if an economy is added

### Acceptance Criteria

- every campaign win advances progression
- rewards visibly change the run deck or resources

## Phase 7: Campaign Prep Screen

### Scope

- Add a player-facing prep shell between nodes.

### Features

- current deck summary
- current roster summary
- encounter preview
- optional deck swap from saved decks
- optional campaign-specific edit rules

### Acceptance Criteria

- player can understand their current run before entering a fight
- encounter information is readable before battle

## Phase 8: Failure and Recovery Rules

### Scope

- Define what defeat means.

### Recommended First Version

Start simple:

- defeat ends the run
- player can immediately retry from campaign menu by continuing or starting over

Possible later additions:

- limited lives
- partial recovery
- checkpoint acts

### Acceptance Criteria

- defeat is explicit and not ambiguous
- campaign state cannot get stuck after a loss

## Phase 9: Final Boss and Campaign Victory

### Scope

- Add a true win condition for the campaign.

### Features

- final encounter
- victory resolution
- campaign clear summary
- completion record

### Acceptance Criteria

- player can complete a full run and see a clear ending state

## Phase 10: Meta Progression and Replayability

### Scope

- Add longer-term reasons to replay campaigns.

### Features

- unlockable starter decks
- unlocked cards or booster pools
- campaign difficulty variants
- alternate enemy routes
- campaign performance stats

### Acceptance Criteria

- a completed campaign meaningfully improves future runs or choice variety

## Battle-System Work Needed Before Production

The campaign loop depends on battle quality. Even if the current rules prototype is functional, these systems should be evaluated explicitly:

### 1. AI Quality

Needed improvements:

- better card play sequencing
- better reveal timing
- awareness of final-climax race state
- encounter-value heuristics

Acceptance target:

- AI should feel intentionally competitive, not random or obviously broken

### 2. Match Readability

Needed improvements:

- cleaner turn and phase signaling
- better reveal and stack resolution readability
- clearer post-encounter result explanation

Acceptance target:

- player can understand why they won or lost an encounter

### 3. Rules Completeness

Needed review areas:

- hidden-support handling
- growth thresholds
- defeat timing
- edge-case reaction windows
- encounter cleanup rules

Acceptance target:

- campaign losses should feel rule-consistent, not prototype-fragile

## Recommended Campaign Economy

For first production pass, keep economy simple.

Recommended approach:

- campaign rewards mostly grant support cards or targeted choices
- do not introduce too many currencies at once
- use one lightweight resource only if needed for healing, rerolls, or shop purchases

Suggested first-pass rule:

- after each win, choose one reward:
  - add a card
  - remove a card
  - upgrade via themed booster

That keeps deck progression meaningful without overcomplicating the economy.

## Recommended Prep Rules

To preserve the campaign identity:

- roster should generally remain stable within a run unless a reward explicitly changes it
- support deck should be the main progression surface
- saved collection decks can seed a run, but a run should then diverge into its own snapshot

This avoids the confusion of:

- “is my campaign editing my permanent deck?”

## Recommended UI Structure

Production campaign likely needs these screens:

1. `Campaign Menu`
- new run
- continue run
- slot list

2. `Campaign Hub`
- current act
- current objective
- next encounters
- deck summary
- rewards and resources

3. `Campaign Prep`
- current deck
- current roster
- encounter preview
- start battle

4. `Battle`
- existing combat screen, refined for production readability

5. `Post-Battle Results`
- victory or defeat
- rewards
- continue

6. `Campaign Victory / Defeat`
- end-of-run summary
- stats
- unlocks

## AI Opponent Design Recommendation

The AI campaign should not just mirror player decks.

Recommended enemy structure:

- Act 1:
  - simpler archetypes
  - readable pacing
- Act 2:
  - stronger reactions
  - more specialized encounter pressure
- Act 3:
  - boss mechanics
  - more punishing synergy

Each named encounter should communicate identity:

- tempo bully
- stall/control enemy
- reveal-heavy trickster
- final boss pressure deck

## Technical Implementation Order

Recommended engineering order:

1. split main menu into `Campaign` and `Skirmish`
2. add campaign save file and slot model
3. add encounter definitions and campaign content loading
4. add campaign screen shell and run state
5. route campaign nodes into battle with AI decks
6. add post-battle results and rewards
7. add campaign progression and final boss completion
8. improve AI and production battle readability
9. add meta progression and replayability

## Proposed Module Structure

Longer-term, campaign should live in its own module cluster:

- `src/campaign/mod.rs`
- `src/campaign/state.rs`
- `src/campaign/content.rs`
- `src/campaign/rewards.rs`
- `src/campaign/progression.rs`
- `src/campaign/persistence.rs`
- `src/campaign/battle_bridge.rs`

Suggested screen additions:

- `src/screens/campaign_menu.rs`
- `src/screens/campaign_hub.rs`
- `src/screens/campaign_results.rs`

## Testing Plan

Add focused tests for:

- campaign run creation and resume
- encounter progression and node unlocks
- saved-deck snapshot into run state
- post-battle reward application
- defeat handling
- final victory completion
- AI encounter entry correctness
- campaign save migration and persistence

Simulation should also expand to support:

- campaign encounter sequences
- reward progression snapshots
- boss win-rate analysis

## Definition of Done For “Production Campaign”

The campaign should be considered production-ready when:

- player can start a campaign from the main menu
- AI opponents and encounters are data-driven
- player controls only their side in campaign mode
- a run persists correctly across sessions
- post-battle rewards meaningfully change the run
- defeat and victory both resolve cleanly
- the player can reach and defeat a final boss
- the loop is understandable without developer knowledge

## Immediate Next Milestone

The best next concrete milestone is:

1. split `Campaign` from `Skirmish`
2. add campaign save slots
3. add data-driven AI encounter definitions
4. launch campaign battles without dual-player setup

That is the equivalent of the deck-builder’s “credible builder” milestone: it turns the current combat prototype into the start of a real player-facing game. 
