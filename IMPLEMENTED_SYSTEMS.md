# Implemented Systems

## Current prototype loop

- Main menu now separates `Single-Player Campaign`, `Configure Match`, and `Open Deck Builder` so the campaign sits alongside the existing skirmish shell instead of replacing it.
- Main menu opens a dedicated match setup screen instead of jumping straight into a hardcoded battle.
- The game now boots fullscreen by default and the main menu includes a saved `Windowed Mode` checkbox that persists through local settings.
- The main menu also includes a direct `Exit Game` action.
- Match setup lets the prototype configure both `Player A` and `Player B`.
- Match setup now shows direct clickable main-character cards and support-pair choices for each roster instead of only cycling through hidden selections.
- Each player currently chooses:
  - a Magical Girl main unit
  - a Magical Girl hidden support pair
  - a Baddie main unit
  - a Baddie hidden support pair
- Starting a match builds runtime state for both players from those selected rosters and support pairs.

## Battle UI

- Shared UI helpers now live under `src/ui/` for colors, 1440p-first layout scaling, and reusable card/panel widgets.
- The prototype window now targets a `2560 x 1440` baseline and the main screens scale from that layout space.
- The menu now links to a deck builder shell as well as match setup.
- The repo now ships procedural card-frame and speed-badge PNG assets under `assets/generated/cards/`, including layered frame pieces, art masks, gloss overlays, badge bases, and badge icons generated from `assets/data/card_visuals.json`.
- The repo now also generates procedural roster portraits, story-card illustration placeholders, and widescreen UI backdrops under `assets/generated/`, with a machine-readable catalog in `assets/data/art_catalog.json`.
- Current UI flows are mouse-driven; the prototype does not require keyboard input to navigate menu, setup, deck builder, or battle actions.
- The battle screen is now player-centric instead of faction-centric.
- The prototype shows `You` and `Enemy` columns with each player's Magical Girl side and Baddie side.
- The active lane is called out explicitly so it is clear whose Magical Girls are attacking whose Baddies.
- `Player A` now sees the actual cards in hand as individual playable buttons instead of a single `play first` shortcut.
- Hand cards and deck-builder cards now render as compact rectangular card tiles with hover previews that draw above the rest of the interface so card text stays readable.
- The battle screen shows visible draw-pile counts for both players.
- The defending player's Prime Baddie is marked when defeated and the finished state shows the winner.
- Campaign battles now use a separate presentation layer that frames the match as `you vs enemy`, and the visible combat panels only show the player's Magical Girls against the AI's Baddies.
- Campaign battles now also hide player-facing Baddie-support reveal controls, so the visible actions stay aligned with the single-player Magical Girl framing.

## Persistence

- Local save files are now versioned and stored under `save/`.
- The prototype loads `profile.json`, `collection.json`, `decks.json`, and `settings.json` with default fallback when files are missing.
- Finishing a match updates the local profile match and win counters and writes the save bundle back to disk.
- Collection ownership now stores counted inventory and still accepts the older array-shaped save format on load.
- The deck builder edits the active local support deck preset and saves changes immediately.
- Local persistence now also stores `campaigns.json`, including active or completed campaign runs, their deck snapshot, encounter progress, and battle history.
- Campaign persistence now supports multiple saved run slots and keeps a selected campaign slot so the player can swap between in-progress, won, or lost runs from the campaign menu.

## Single-player campaign

- The game now loads a data-driven `Magical Girl Rising` campaign from `assets/data/campaigns/`.
- Starting a campaign creates a persistent run that snapshots the currently selected saved support deck; if no saved deck is selected, the first starter template is used as a fallback seed deck.
- Campaign battles always place the player on `Player A` and feed the opposing AI side from encounter-defined starter loadouts.
- The campaign menu supports starting a new run, continuing the selected in-progress run, abandoning the selected in-progress run, and reviewing how many campaign clears have been recorded locally.
- The campaign menu now shows explicit save slots for every stored run, lets the player select which slot to inspect or continue, and starts new runs without overwriting older slots.
- The campaign hub shows the current encounter, run deck, roster snapshot, recent reward cards, and progression before launching the next battle.
- The campaign hub now also lets the player explicitly choose exactly two Magical Girls from the roster as that run's current supports before an encounter can begin.
- Winning an encounter advances the run to the next node, adds the encounter's first configured reward card directly to the run deck snapshot, and preserves that upgraded deck for later battles in the same run.
- Winning the final encounter marks the run as cleared and returns the player to the campaign menu with a completion notice.
- Losing a campaign battle marks that run slot as lost and returns the player to the campaign menu without affecting the existing skirmish setup flow.

## Deck builder shell

- A dedicated deck builder screen now supports an explicit saved-deck list instead of only editing one implicit active starter deck.
- Saved decks can now be created, selected, renamed, duplicated, and deleted directly in the builder.
- Starter loadouts now act as reusable templates with explicit `Create` actions that generate editable saved decks without mutating the template itself.
- Saved decks now record optional template origin metadata so copied starter-based decks stay distinct from custom decks.
- Match setup can now assign the currently selected saved support deck to `Player A` or `Player B`, and starting a battle uses those assigned saved decks instead of always falling back to the prototype defaults.
- The deck builder now has a separate roster-edit layer for the selected saved deck, with Magical Girl and Baddie rosters kept distinct from support-card editing.
- The right-side rail now includes a permanent deck summary panel that shows legal or illegal status, support-card count, Magical Girl roster count, Baddie roster count, collection completeness, and current warnings.
- Deck validation now reports support-count problems, duplicate support-card copy-limit violations, duplicate roster entries, and missing owned cards separately.
- The support-card browser now has a live search bar with result counts and soft-failing query tags for `speed`, `align`, `type`, `owned`, `missing`, and `copies`.
- The support-card browser now also exposes toggleable filter buttons for speed, alignment, card type, owned, missing, in-deck, and not-in-deck states, plus removable active-filter chips and a `Clear All` action.
- The support-card browser now exposes visible sort, group, and view controls, with sorting applied after search and filtering, grouping headers for alignment, speed, or card type, and both grid and compact-list card views.
- The editor layout now uses three explicit tabs for `Support Cards`, `Magical Girl Roster`, and `Baddie Roster` instead of a shared roster tab.
- The right rail now keeps both preview content and current deck contents visible at the same time, so support-card counts or active roster slots stay readable while browsing the collection.
- Roster editing now uses the full center browser area for one side at a time, with the visible right-rail slots acting as the swap targets for that side.
- Starter templates now surface their own playstyle, description, roster seed, and support-card seed in the deck-builder preview, and their left-rail action is an explicit `Create Deck` flow that always produces a new editable saved deck.
- The deck builder can now export the selected saved deck to a portable `EH1` deck-code string and import that code into a new editable saved deck, with in-screen copy and paste actions for sharing.
- Imported or incomplete decks now highlight missing support cards directly in the browser and current-deck rail, show a dedicated missing-count summary, and surface owned replacement suggestions that prioritize matching speed, alignment, and card type.
- Saved decks now carry deck notes, archetype tags, created and updated timestamps, and recent-card history; the builder exposes metadata editing plus `Undo` and template-based `Reset` actions, and the saved-deck list now marks recently modified decks.
- The screen can open a `10`-card booster that rolls from the full Magical Girl, Baddie, and story-card pool and records those results in local collection counts.
- The screen can add or remove story cards on the selected saved deck while respecting configured deck size, per-card copy limits, and owned story-card copies in collection.
- The deck builder now uses a fixed grid plus a dedicated preview panel so the full card list fits the 1440p layout without overlapping columns.
- Hovering a booster-result row now previews the opened card in the same right-side panel, and clicking a starter template row previews that template without creating a deck.

## AI simulation

- A headless AI-vs-AI simulation runner now exists for balance testing.
- Simulations can run starter-loadout matchups for both starting-player orders without opening the game window.
- The project now exposes a `simulate` binary that prints a JSON batch report, saves the same report to `simulation_report.json`, and includes winners, rounds completed, Final Climax usage, failed Final Climax counts, round-cap or action-cap exits, milestone rounds, per-action traces with played card names and reaction passes, and per-encounter timelines with power and stage snapshots.
- The current starter tuning includes a Velvet-specific starter rebuild: `Velvet Ambush` now leans on higher Prime Baddie power cards plus a smaller package of proactive Magical Girl setup and burst cards, and its Baddie support pair now uses a slightly lower defensive ceiling to avoid mirror-match walling.
- The prototype story card pool now also includes `Synchronized Finish`, a generic Magical Girl encounter closer that rewards already-revealed support pressure.

## Hidden support behavior

- Each player keeps full five-character Magical Girl and Baddie rosters in runtime state.
- Each roster enters battle with exactly two selected supports.
- Only the first selected support starts revealed in the prototype.
- The second selected support remains hidden until a reveal effect or reveal action resolves.
- Hidden supports do not contribute Power until revealed.

## Timing and reactions

- Main-phase card plays now come from a specific hand card, and manual support reveals still enter the same timing system.
- Reaction cards and reaction-speed support reveals can be added on top of the current stack.
- Stack resolution is newest-to-oldest after both players pass priority.
- The battle screen shows whether the reaction window is open and displays a rolling event log of queued and resolved actions.
- Daily Life now uses shared alternating proactive priority instead of giving the whole phase to the lane attacker.
- Encounter now uses the same alternating proactive priority structure, but each player is still limited to one proactive Encounter card play per round.
- After a proactive card or reveal is queued, the opposing player receives reaction priority before the stack resolves.
- Each player may reveal at most one support total per round, even if one reveal happens in Daily Life and the other support remains hidden for Encounter.

## Encounter prototype rules

- The active player alternates by round.
- During an Encounter, the active player's Magical Girls attack the opposing player's Baddies.
- Each player now opens with `5` cards and there is no extra automatic draw before the first `Daily Life` action window.
- Both players draw `2` simultaneously at the start of each later `Daily Life` round.
- At the end of each round, both players automatically discard down to a hand size of `7`.
- Daily Life ends when both players pass consecutively with no reaction chain active.
- Encounter resolves automatically when both players pass consecutively with no reaction chain active.
- Encounter Power only counts main units plus revealed, non-exhausted supports on the engaged lane.
- Encounter rewards now use configurable `+3 / +1 / +1` win, loss, and tie growth values from `assets/data/rules/match_rules.json`.
- Story cards no longer directly add, remove, or reduce `Radiance` or `Dread`; those stats now come from encounter results rather than card text.
- Final Climax can be declared once the active player's Main Magical Girl reaches `Radiant`.
- Failed Final Climax attempts now grant the attacking Main Magical Girl a cumulative `+1` Final Climax Power bonus on both draws and losses, so repeated Final Climax stalls eventually break.
- Final Climax loss exhaustion now only applies while the opposing Main Magical Girl is not yet `Radiant`; if both mains are already `Radiant`, failed Final Climax attempts do not exhaust either side.
- Winning Final Climax defeats the defending player's Prime Baddie and ends the match.
