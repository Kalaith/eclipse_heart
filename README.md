# Eclipse Heart

Eclipse Heart is a card battler and deck-building game about magical-girl teams, rival baddies, hidden supports, and dramatic battle timing.

You manage rosters, build support decks, and choose when to commit resources during campaign or skirmish fights.

## Gameplay

- Build and tune support decks.
- Manage magical-girl and baddie rosters.
- Choose cards, battle actions, and timing windows.
- Watch for hidden supports and final-climax opportunities.
- Balance short-term survival against long-term setup.

## Goal

Win battles by reading the board, choosing the right support at the right time, and building a roster that can handle escalating encounters.

## Controls

- Mouse: select cards, menus, and battle actions.
- Esc: pause, back out, or close dialogs.
- Enter: confirm text entry where available.

## Current Scope

Playable card-battle foundation with campaign and skirmish flow, support decks, rosters, and tactical battle choices.

## Documentation

Project notes, plans, rules references, and implementation records live in [`docs/`](docs/).
# Practical Future Improvements

- Add card-state tests for draw, discard, deck editing, battle setup, campaign reward, and roster-dialog side effects.
- Introduce a combat replay fixture that can reproduce battle bugs from seed, deck, enemy, and starting hand data.
- Move card, enemy, campaign, and reward tuning into validated data tables to reduce rule drift between screens.
- Separate deck-builder preview and browser state from mutation commands so UI filtering cannot alter deck contents accidentally.

