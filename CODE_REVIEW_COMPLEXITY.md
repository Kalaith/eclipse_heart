# Eclipse Heart Code Complexity Review

This file captures the code review and complexity improvement recommendations for `eclipse_heart`.

## 1) `Game::update` is monolithic (src/game.rs)
- The `match screen_action` block has 30+ arms with behavior, persistence and state modification.
- Suggested: refactor into an action handler system (methods per action / `ScreenAction::apply(&mut AppState)`), reduce cyclomatic complexity and make route table easy to test.

## 2) `open_booster` performance and allocation cost (src/game.rs)
- Rebuilds the entire pool and clones entries each call.
- Suggested: keep a prebuilt pool in state, use rand helpers (`SliceRandom`) to pick, avoid repeated deep copy.

## 3) `MatchState` large file with mixed responsibilities (src/state/match_state.rs)
- includes setup, runtime rules, utility lookups, and lifecycle transitions.
- Suggested: split state into `match_state.rs`, `match_setup.rs`, `character.rs`, `player_state.rs` and simple helper modules.

## 4) Duplicate accessor methods for players and sides (src/state/match_state.rs)
- `player_a_mg_main_name`, `player_b_baddie_support_names`, etc.
- Suggested: generic side-access helpers `main_name(player, bool)` + single method for support names.

## 5) `MatchEngine::apply_card_effect` is too large (src/engine/match_engine.rs)
- 20+ match arms handling resource effects, power effects, reveal, exhaust, stage changes.
- Suggested: separate card effect handlers and/or method on `CardEffect` to reduce function complexity.

## 6) `resolve_encounter` has many responsibilities (src/engine/match_engine.rs)
- outcome resolution, growth application, final climax mutation, win condition, event logs.
- Suggested: break into helper methods: `outcome_of`, `apply_growth`, `apply_final_climax_effects`.

## 7) Reaction stack resolution complexity (src/engine/match_engine.rs)
- `resolve_stack` plus root action special-case `finish_root_action`.
- Suggested: separate stack item resolution method and phase-finalization paths.

## 8) Repeated UI action condition blocks (src/screens/battle.rs, src/screens/deck_builder.rs)
- lots of boolean guards with action_button calls.
- Suggested: abstract action widget builder with condition -> action mapping.

## 9) String label logic for phase/stage (src/state/match_state.rs & UI)
- `current_phase_label` and debug formatting are mixed.
- Suggested: implement `Display` for `MatchPhase`, `CharacterStage` to centralize naming.

## 10) `support_pairs` allocates vector each call (src/state/match_state.rs)
- called in cycles/lookup and may be expensive.
- Suggested: cache options or provide iterator-based support pair generation.

## 11) `appended_events` is O(N²) in worst case (src/engine/simulation.rs)
- overlap scanning for event diff.
- Suggested: maintain event index or use a delta collection approach.

## 12) UI layout constants repeated (src/screens/*.rs)
- magic coordinates repeated for each screen.
- Suggested: use constants or a style config/layout helper object.

## 13) Hand limit discard logic hidden in `ready_end_of_round` (src/state/match_state.rs)
- state changes and event logs inside a long flow.
- Suggested: extract and unit test `discard_down_to_hand_limit` separately.

## 14) Test coverage guidance
- add specific tests for:
  - reaction priority + stack resolves
  - final climax defeat flow
  - support reveal preconditions
  - `MatchEngine::pass_*` edge conditions

## 15) Possible coding standards updates
- enforce `clippy::cognitive_complexity` and ensure each function under threshold.
- align with existing `CODE_STANDARDS.md` for “small focused functions”.

---

### Next step
1. Implement refactor in small PR chunks.
2. Add/adjust tests as behavior is restructured.
3. Run `cargo fmt`, `cargo clippy`, and `cargo test`.
