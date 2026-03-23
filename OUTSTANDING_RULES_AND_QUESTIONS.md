# Eclipse Heart Outstanding Rules And Questions

This file tracks what is still unresolved after the shift to the `Radiance / Dread`
growth model.

## Resolved rules

### Deck construction

- Support deck size is `40`.
- Story card copy limit is `3`.
- Copy limit is universal.
- Starter decks are partially editable and include `1 booster`.
- Decks are independent of the roster.

### Roster construction

- Roster size is exactly `5 Magical Girls` and `5 Baddies`.
- Players show all `10` roster cards to the opponent.
- Each player chooses their `Main MG` and `Prime Baddie`.
- After that, each player chooses `2 Support MGs` and `2 Support Baddies` and places them hidden.
- Magical Girls and Baddies are singleton roster units.
  - No duplicates are allowed.
  - Alternate-name or alternate-ability versions of the same character still count as duplicates.
- There are currently no faction, school, personality, or partner restrictions.

### Match setup

- Opening hand size is `5`.
- There is no mulligan.
- First player is decided by `heads or tails`.
- First player still draws at the start.
- Players start with no resources unless granted by setup text.
- Support units always start hidden.

### Core turn structure

- Core structure:
  - `Daily Life -> Encounter -> repeat -> Final Climax`
- Reactions are optional.
- Players can react to reactions.
- Chains resolve after the reaction sequence completes.
- Trigger resolution order is `newest to oldest`.
- Supports can reveal whenever legal for their text.
- Support reveals can be actions or reactions.
- Supports can reveal in response to other supports revealing.
- Encounter cards may be played during Daily Life if their effect still matters there.

### Power and growth stats

- Magical Girls and Baddies both have `Power`.
- Magical Girls use `Radiance`.
- Baddies use `Dread`.
- `Radiance` and `Dread` are fully per-character.
- `Spotlight`, `Hope`, `Corruption`, `Bond`, `Strain`, and `Exposure` are not part of the current rules model.
- `Radiance` and `Dread` can never go below zero.

### Character progression

- Every character, including supports, has two thresholds.
- Magical Girls:
  - first threshold -> `Transformed`
  - second threshold -> `Radiant`
- Baddies:
  - first threshold -> `Awakened`
  - second threshold -> `Catastrophe`
- Characters can continue progressing all the way to final form, including supports.
- Thresholds are per character and can vary.
  - Example: `3 / 6` for a fast weaker unit
  - Example: `5 / 10` for a slower stronger unit
- Upgrades happen immediately once the effect fully resolves and the threshold is reached.
- Upgrades do not happen as reactions.
  - Reactions can still stop or reduce the gain before resolution.
- Progress resets after an upgrade.
  - Excess gain above the threshold is lost.
- Once a final form is reached, it stays unlocked unless a rare card explicitly downgrades it.
- A character can progress multiple times in one Encounter if enough gains are generated.
- If a fully transformed character would gain “final-form points” from a tie, those points are not used.
  - This applies to fully transformed Magical Girls and fully transformed Baddies.

### Encounter growth rewards

- Winner gains `+3`.
- Loser gains `+1`.
- Tie gives both `+2`.
- Magical Girls gain `Radiance`.
- Baddies gain `Dread`.
- Baddies still gain `+1 Dread` even when losing.
- Cards and abilities can modify these gains.
- It is possible to transform during an Encounter and continue gaining toward the next stage in that same Encounter.
- Spending too many cards to overshoot a threshold is inefficient because extra gain is lost on upgrade.

### Final-form persistence

- Transformed / Awakened / Radiant / Catastrophe abilities remain active once unlocked unless a card explicitly removes or downgrades them.

## Still pending

### Final Climax legality

- Final Climax can be declared only at the start of an Encounter.
- The `Main MG` may declare Final Climax if she is fully transformed.
- The opposing villain cannot stop the declaration unless a specific card explicitly says so.

### Prime Baddie defeat

- During Final Climax, if the Prime Baddie's Power is lower when the Encounter ends, it is defeated.
- A win is a win.
- On a Final Climax tie, no defeat occurs.
- A tie gives points, except fully transformed characters cannot use those tie points.
- If a Main MG is defeated in Final Climax, it becomes exhausted for one turn and does not count its Power during the next Encounter.
- No non-Final-Climax defeat condition is currently defined.

### Encounter comparison details

- Hidden supports contribute nothing until revealed.
- Encounter Power is:
  - `Main Magical Girl Power + all revealed, non-exhausted Support Magical Girls`
  - versus
  - `Prime Baddie Power + all revealed, non-exhausted Support Baddies`

### Reveal and hidden-information targeting

- Hidden supports cannot be targeted directly.
- A card can only interact with a hidden support if it explicitly says it can reveal a card.
- After setup, the opponent does not know which support units were chosen.
- The opponent only knows the original full `5 + 5` roster reveal.

### Exhaust / ready rules

- Exhaustion lasts for one full round by default.
- If a unit is exhausted during Daily Life, it does not add Power to the next Encounter.
- If a unit becomes exhausted mid-Encounter, it does not add Power for that Encounter.
- If exhaustion is applied at the end of an Encounter, it lasts through the end of the next Encounter.
- Exhaustion blocks both Power contribution and abilities.

### Anti-stall rule

- Current decision: no hard anti-stall rule for now.
- Reasoning:
  - both players are managing two teams
  - smart card play should naturally create openings
  - overcommitting to one side weakens the other

## Highest-priority blockers

1. Finalize exact wording for rare downgrade and upgrade interactions
2. Finalize exact wording for card-specific hidden-support exceptions
3. Finalize exact wording for card-specific exhaustion-duration exceptions
