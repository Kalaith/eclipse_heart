# TODO — Eclipse Heart

## Bugs

- Campaign start can panic with `starter loadout references invalid support pair`: `support_pair_index_for_ids` in `src/state/match_state_setup.rs` only matches ascending-index pairs, but the campaign hub hands it the player's two chosen supports in selection order.
- Deck-select and campaign-start screens overlap elements at the 1440p baseline; the title and main menu do not.
- Setup and side-select give no way to inspect a Magical Girl or Baddie before committing to it.

## Campaign

- Grant a reward choice after each encounter: encounters already list three `reward_story_card_ids` but only the first is applied.
- Add an explicit post-battle results screen (victory/defeat, rewards, continue) instead of returning straight to the campaign menu.
- Give the campaign a narrative frame; encounters carry only `intro_text` and the run has no story arc.
- Structure the campaign into acts with a boss per act — `magical_girl_campaign.json` is currently three linear nodes.
- Meta progression across runs: unlockable starters, difficulty variants, alternate routes, run statistics.
- Improve campaign AI: card-play sequencing, reveal timing, and awareness of the Final Climax race.

## Content and rules

- Character definitions carry only power and threshold numbers; reveal, transformed, and final-form abilities are still unimplemented.
- Settle the rare-case wordings listed in `docs/RULES.md` §16 (simultaneous upgrade/downgrade, Final Climax replacement effects, card-specific hidden-support and exhaustion exceptions).

## Engineering

- `src/game.rs`, `src/screens/battle.rs`, `src/engine/match_engine.rs`, and `src/state/decks.rs` all sit just under the 800-line limit and need splitting before they grow further.
- Add card-state tests for draw, discard, deck editing, battle setup, campaign reward, and roster-dialog side effects.
- Add a combat replay fixture that reproduces a battle from seed, deck, enemy, and starting hand.
- Implement `Display` for `MatchPhase` and `CharacterStage` so phase and stage labels are defined in one place.
- Separate deck-builder preview and browser state from mutation commands so filtering cannot alter deck contents.
