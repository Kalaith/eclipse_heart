# Eclipse Heart Build Plan

## Current status

The project has design notes and deployment scaffolding, but no Rust crate or gameplay code yet.

## Phase 1: Foundation

- [x] Review `AGENTS.md`, repo guides, and `raw_notes.md`
- [x] Write project-specific PRD, tech stack, and implementation plan
- [x] Create `eclipse_heart` Cargo crate
- [x] Add crate to workspace root
- [x] Create `src/` module tree
- [x] Create `assets/data/` tree
- [x] Add empty app shell that builds for native

## Phase 2: Rules core

- [x] Define match phases
- [x] Define player actions and event log enums
- [x] Define runtime state for Main MG, supports, Prime Baddie, and support baddies
- [x] Implement Daily Life action flow
- [x] Implement Encounter resolution
- [x] Implement `Radiance` and `Dread` thresholds
- [x] Implement transform and awaken checks
- [x] Implement Final Climax trigger
- [x] Implement Prime Baddie defeat condition

## Phase 3: Content pipeline

- [x] Define JSON schemas for Magical Girls
- [x] Define JSON schemas for Baddies
- [x] Define JSON schemas for story cards
- [x] Encode the current 20 story cards
- [x] Encode the current 5 Magical Girls
- [x] Encode the current 5 Baddies
- [ ] Encode Yuki and Noctra full upgrade ladders

## Phase 4: Playable client

- [ ] Main menu
- [ ] Collection screen
- [ ] Deck builder
- [ ] Roster selection
- [ ] Battle screen
- [ ] Result screen
- [ ] Battle log UI

## Phase 5: Persistence and tests

- [ ] Save collection and decks
- [ ] Save roster presets
- [ ] Add loader tests
- [ ] Add phase transition tests
- [ ] Add Encounter resolution tests
- [ ] Add progression threshold tests

## Phase 6: Later systems

- [x] AI opponent
- [ ] local simulated trade flow
- [ ] online account and trade authority
- [ ] online PvP authority

## Build discipline

Run after each implementation slice:

```bash
cargo fmt
cargo clippy -p eclipse_heart --all-targets --all-features
cargo test -p eclipse_heart
cargo build --release -p eclipse_heart
```

## Open blockers

- support deck size is not fully settled in the notes
- mulligan rules are not settled
- support-unit upgrade rules are not fully settled

## Revision

- 2026-03-21: replaced generic template with Eclipse Heart build plan
