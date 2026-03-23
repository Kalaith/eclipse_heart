# Implemented Systems

## Current prototype loop

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
- Current UI flows are mouse-driven; the prototype does not require keyboard input to navigate menu, setup, deck builder, or battle actions.
- The battle screen is now player-centric instead of faction-centric.
- The prototype shows `You` and `Enemy` columns with each player's Magical Girl side and Baddie side.
- The active lane is called out explicitly so it is clear whose Magical Girls are attacking whose Baddies.
- `Player A` now sees the actual cards in hand as individual playable buttons instead of a single `play first` shortcut.
- Hand cards and deck-builder cards now render as compact rectangular card tiles with hover previews that draw above the rest of the interface so card text stays readable.
- The battle screen shows visible draw-pile counts for both players.
- The defending player's Prime Baddie is marked when defeated and the finished state shows the winner.

## Persistence

- Local save files are now versioned and stored under `save/`.
- The prototype loads `profile.json`, `collection.json`, `decks.json`, and `settings.json` with default fallback when files are missing.
- Finishing a match updates the local profile match and win counters and writes the save bundle back to disk.
- Collection ownership now stores counted inventory and still accepts the older array-shaped save format on load.
- The deck builder edits the active local support deck preset and saves changes immediately.

## Deck builder shell

- A dedicated deck builder screen now exists for the active support deck prototype.
- The screen can load a starter support deck into the active preset.
- Starter rows now use matched visual and clickable rectangles, and each row has an explicit `Edit` button that opens a persistent editable preset for that starter instead of resetting it each time.
- The deck builder now has a separate roster-edit layer for the active preset, with saved Magical Girl and Baddie rosters kept distinct from support-card editing.
- The screen can open a `10`-card booster that rolls from the full Magical Girl, Baddie, and story-card pool and records those results in local collection counts.
- The screen can add or remove story cards while respecting configured deck size, per-card copy limits, and owned story-card copies in collection.
- The deck builder now uses a fixed grid plus a dedicated preview panel so the full card list fits the 1440p layout without overlapping columns.
- Hovering a booster-result row now previews the opened card in the same right-side panel, and clicking a starter row shows that starter deck's card list without loading it.
- The active support deck is created automatically from the first starter loadout if no local deck preset exists yet.

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
