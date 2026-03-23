# Eclipse Heart Rules

This document is the current formal rules reference for Eclipse Heart.

## 1. Game Structure

Eclipse Heart is a 1v1 card game where each player controls:

- a Magical Girl team
- a Baddie team
- a support deck

Each player is trying to defeat the opposing `Prime Baddie`.

## 2. Match Construction

Each player brings:

- `5` Magical Girls
- `5` Baddies
- `1` support deck of `40` cards

Deck rules:

- support deck size is exactly `40`
- maximum `3` copies of the same story card
- copy limit is universal
- the support deck is independent of the roster

Roster rules:

- Magical Girls and Baddies are singleton roster units
- no duplicate characters are allowed
- alternate versions of the same character still count as duplicates

## 3. Match Setup

At the start of the match:

1. Each player reveals all `10` roster cards:
   - `5` Magical Girls
   - `5` Baddies
2. Each player chooses their:
   - `Main Magical Girl`
   - `Prime Baddie`
3. Each player then chooses:
   - `2` Support Magical Girls
   - `2` Support Baddies
4. Those support units are placed hidden.

Hidden support information:

- after setup, the opponent has no information about which support units were chosen
- the opponent only knows the original full `5 + 5` roster reveal

Opening hand and start rules:

- each player draws `4` cards
- there is no mulligan
- first player is decided by heads or tails
- after setup, both players begin with those `4` cards in hand
- before the first `Daily Life` action window, both players draw `1` card at the same time
- before each later new `Daily Life` round starts, both players also draw `1` card at the same time
- players start with no resources unless a card explicitly grants them
- players discard down to `7` cards in hand at the end of each round

## 4. Core Flow

Match flow is:

1. `Daily Life`
2. `Encounter`
3. repeat `Daily Life -> Encounter`
4. `Final Climax`

The game does not end automatically when Final Climax begins.
The game ends when a `Prime Baddie` is defeated.

## 5. Card Speeds And Timing

Cards and effects may operate in different windows.

Current timing principles:

- reactions are optional
- players can react to reactions
- chains resolve after the reaction sequence completes
- trigger resolution order is `newest to oldest`
- in the current implementation, the active player takes the main action window for the round
- that player either plays a legal card from hand or passes
- if a card or reveal is played, the opposing player gets a reaction window before the stack resolves

Supports:

- each player may reveal at most `1` support total per round
- supports may reveal whenever their text legally allows within that round limit
- support reveals can function as actions or reactions
- supports may reveal in response to other supports revealing

Encounter cards:

- Encounter cards may be played during `Daily Life` if their effect still matters there
- if an Encounter card would have no meaningful effect in Daily Life, then playing it there is pointless but not a separate rules problem

## 6. Power

Both Magical Girls and Baddies have `Power`.

Encounter Power is calculated as:

- `Main Magical Girl Power + all revealed, non-exhausted Support Magical Girls`
- versus
- `Prime Baddie Power + all revealed, non-exhausted Support Baddies`

Hidden supports contribute nothing until revealed.

Exhausted units contribute no Power.

## 7. Growth Stats

Magical Girls use `Radiance`.
Baddies use `Dread`.

Rules for growth stats:

- `Radiance` and `Dread` are tracked per character
- every character has its own thresholds
- `Radiance` and `Dread` can never go below zero

These stats replace the earlier `Bond`, `Strain`, and `Exposure` concepts.

## 8. Character Progression

Every character, including supports, has two progression thresholds.

Magical Girl stages:

- `Base`
- `Transformed`
- `Radiant`

Baddie stages:

- `Base`
- `Awakened`
- `Catastrophe`

Threshold model:

- first threshold unlocks `Transformed` or `Awakened`
- second threshold unlocks `Radiant` or `Catastrophe`
- thresholds can vary by character

Examples:

- a fast weaker character may use `3 / 6`
- a slower stronger character may use `5 / 10`

Upgrade rules:

- upgrades happen immediately once the effect fully resolves and the threshold is reached
- upgrades do not happen as reactions
- reactions may still stop or reduce the gain before resolution
- progress resets after an upgrade
- excess gain above the threshold is lost
- once final form is reached, it stays unlocked unless a card explicitly downgrades it
- a character may progress multiple times in one Encounter if enough gain is generated

Support progression:

- Support Magical Girls can become `Transformed` and `Radiant`
- Support Baddies can become `Awakened` and `Catastrophe`

## 9. Encounter Growth Rewards

At the end of an Encounter:

- the winner gains `+3`
- the loser gains `+1`
- on a tie, both gain `+1`

Application:

- Magical Girls gain `Radiance`
- Baddies gain `Dread`

Additional rules:

- Baddies still gain `+1 Dread` even when losing
- direct card effects no longer add, remove, or reduce `Radiance` or `Dread`
- cards and abilities may still affect Power, exhaustion, and reveal timing
- if a fully transformed character would receive tie points toward further growth, those points do not apply
- overshooting a threshold wastes the extra gain because progress resets on upgrade

## 10. Hidden Supports

Hidden supports follow these rules:

- hidden supports cannot be targeted directly
- cards can only interact with hidden supports if they explicitly say they can reveal or affect hidden cards
- hidden supports contribute nothing until revealed

## 11. Exhaustion

Exhaustion blocks both:

- Power contribution
- abilities

Default exhaustion timing:

- if a unit is exhausted during `Daily Life`, it does not add Power to the next Encounter
- if a unit becomes exhausted mid-Encounter, it does not add Power for that Encounter
- if exhaustion is applied at the end of an Encounter, it lasts through the end of the next Encounter

In short, exhaustion lasts for one full round by default unless a card says otherwise.

## 12. Final Climax

Final Climax is a special Encounter.

Declaration rules:

- Final Climax can be declared only at the start of an Encounter
- the `Main Magical Girl` may declare Final Climax if she is fully transformed
- the opposing villain cannot stop the declaration unless a specific card explicitly says so

## 13. Prime Baddie Defeat

The `Prime Baddie` can be defeated only during `Final Climax`.

Defeat rule:

- if it is Final Climax and the Prime Baddie's Power is lower when the Encounter ends, that Prime Baddie is defeated

Tie handling:

- on a Final Climax tie, no defeat occurs
- ties may still give points, but fully transformed characters cannot use those tie points
- each failed Final Climax attempt that ends in a tie gives the attacking Main Magical Girl a cumulative `+1 Power` bonus for later Final Climax attempts

If a `Main Magical Girl` loses Final Climax:

- that Main Magical Girl becomes exhausted for one turn if the opposing player's Main Magical Girl is not already `Radiant`
- that Main Magical Girl does not count its Power during the next Encounter
- that failed Final Climax attempt also gives the attacking Main Magical Girl a cumulative `+1 Power` bonus for later Final Climax attempts
- if both players' Main Magical Girls are already `Radiant`, failed Final Climax losses do not cause exhaustion

## 14. Anti-Stall

Final Climax now includes a hard anti-stall rule:

- every failed Final Climax attempt that ends in a draw or in a loss for the attacking Main Magical Girl grants that Main Magical Girl a cumulative `+1 Power` bonus for later Final Climax attempts
- these bonuses stack across repeated failed Final Climax attempts
- once both Main Magical Girls are `Radiant`, Final Climax losses no longer apply the exhaustion catch-up penalty
- the intent is that repeated failed Final Climax attempts must eventually break the stall

## 15. Current Design Notes

These are current product assumptions, not narrow mechanical edge-case rules:

- starter decks are editable and include `1` booster
- there are currently no faction, school, personality, or partner restrictions
- support decks do not need to match roster identities

## 16. Open Edge Cases

The core rules are currently settled, but implementation may still need exact wording for rare cases such as:

- simultaneous downgrade and upgrade interactions
- simultaneous Final Climax defeat replacement effects
- card-specific exceptions to hidden-support rules
- card-specific exceptions to exhaustion duration
