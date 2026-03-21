# Eclipse Heart PRD

## Overview

Eclipse Heart is a 1v1 digital collectible card game built in Rust. Each player pilots both a Magical Girl side and a Baddie side. The core tension is split-front play: Magical Girls win Encounters to expose and defeat the opposing Prime Baddie, while Baddies pressure the opposing Main Magical Girl with Strain.

`raw_notes.md` is the current design source of truth. This PRD summarizes the most stable decisions from those notes.

## Core product shape

Each player brings:

- 5 Magical Girls
- 5 Baddies
- 1 support deck of story cards

Each match uses:

- 3 chosen Magical Girls from the 5
- 1 chosen Prime Baddie
- 2 chosen Support Baddies

At match start:

- both rosters are visible
- the Main Magical Girl and Prime Baddie are face-up
- chosen supports begin hidden

## Core features

### P0: Playable match loop

- Setup phase
- Daily Life phase with alternating actions
- Reaction and counter-reaction windows
- Encounter phase with power comparison
- Final Climax trigger and resolution
- Win by defeating the opposing Prime Baddie

### P0: Character progression

- Magical Girl progression: `Base -> Transformed -> Radiant`
- Baddie progression: `Base -> Awakened -> Catastrophe`
- Before Final Climax only the first upgrade is available
- Final Climax unlocks the final upgrade

### P0: Pressure systems

- `Strain` tracks pressure on the Magical Girl side
- `Exposure` tracks pressure on the Baddie side
- `Bond` is a per-Magical-Girl resource
- `Dread` is a per-Baddie resource

Prototype thresholds from the notes:

- Transform at 6 Strain
- Awaken at 6 Exposure
- Bond cap 3
- Dread cap 3

### P0: Deck and roster building

- players can save support decks
- players can save roster presets
- deck contents are story cards, not Magical Girl or Baddie roster units

### P1: Collection and progression

- owned card collection
- starter loadouts
- unlock and reward flow

### P1: Online competitive support

- account-backed deck submission
- online match authority
- trading authority

## UX goals

- readable board state before flashy visuals
- hidden supports create tension without becoming unclear
- phase timing must be explicit on screen
- event log must explain what changed and why

## Out of scope for first playable

- bundle shop
- booster economy
- advanced cosmetics
- fusion system
- ranked service features

## Open design gaps

- final support deck size
- mulligan rules
- exact Prime Baddie defeat procedure
- support upgrade rules outside the Main/Prime path

## Revision

- 2026-03-21: initial Eclipse Heart PRD draft based on `raw_notes.md`
