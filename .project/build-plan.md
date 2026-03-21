# Eclipse Heart Build Plan

## Current status

The project has design notes and deployment scaffolding, but no Rust crate or gameplay code yet.

## Phase 1: Foundation

- [x] Review `AGENTS.md`, repo guides, and `raw_notes.md`
- [x] Write project-specific PRD, tech stack, and implementation plan
- [ ] Create `eclipse_heart` Cargo crate
- [ ] Add crate to workspace root
- [ ] Create `src/` module tree
- [ ] Create `assets/data/` tree
- [ ] Add empty app shell that builds for native

## Phase 2: Rules core

- [ ] Define match phases
- [ ] Define player actions and event log enums
- [ ] Define runtime state for Main MG, supports, Prime Baddie, and support baddies
- [ ] Implement Daily Life action flow
- [ ] Implement Encounter resolution
- [ ] Implement Strain and Exposure thresholds
- [ ] Implement transform and awaken checks
- [ ] Implement Final Climax trigger
- [ ] Implement Prime Baddie defeat condition

## Phase 3: Content pipeline

- [ ] Define JSON schemas for Magical Girls
- [ ] Define JSON schemas for Baddies
- [ ] Define JSON schemas for story cards
- [ ] Encode the current 20 story cards
- [ ] Encode the current 5 Magical Girls
- [ ] Encode the current 5 Baddies
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

- [ ] AI opponent
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
