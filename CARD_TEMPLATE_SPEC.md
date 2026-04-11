# Card Template Spec

This document defines the fixed outer card template system for Eclipse Heart.

## Template Model

The game uses `3` base frame families:

- `magical_girl`
- `baddie`
- `neutral`

Card speed is a separate overlay, not a separate full template:

- `daily_life`
- `reaction`
- `encounter`

This keeps the card shell stable while still making timing readable at a glance.

## Canvas

Master design size:

- width: `750`
- height: `1050`
- safe margin: `36`

All templates share the same outer size and internal layout zones.

## Shared Layout Zones

These zones must stay aligned across all three template families.

1. `name_bar`
- `x: 52`
- `y: 42`
- `w: 546`
- `h: 74`

2. `speed_badge`
- `x: 618`
- `y: 42`
- `w: 80`
- `h: 80`

3. `art_frame`
- `x: 52`
- `y: 136`
- `w: 646`
- `h: 410`

4. `type_row`
- `x: 52`
- `y: 566`
- `w: 646`
- `h: 42`

5. `rules_box`
- `x: 52`
- `y: 628`
- `w: 646`
- `h: 276`

6. `flavor_box`
- `x: 52`
- `y: 916`
- `w: 646`
- `h: 52`

7. `footer_meta`
- `x: 52`
- `y: 980`
- `w: 646`
- `h: 24`

## Visual Direction

### `magical_girl`

- optimistic, luminous, graceful
- rounded separators and soft arcs
- crescents, stars, ribbons, stage-light bloom
- palette anchor: ivory, sky, coral, gold

### `baddie`

- theatrical, hostile, dramatic
- sharper separators and angular trims
- thorns, broken glass, curtains, watchful eye motifs
- palette anchor: charcoal, crimson, magenta, venom-teal

### `neutral`

- tactical, shared-system, less character-owned
- restrained ornament and cleaner panel lines
- palette anchor: parchment, brass, slate, muted red

## Speed Badge

Each speed uses the same badge zone and badge geometry.

### `daily_life`

- short label: `Daily`
- intended read: calm setup / non-combat play

### `reaction`

- short label: `React`
- intended read: interrupt / response timing

### `encounter`

- short label: `Encounter`
- intended read: clash / combat timing

The speed badge should change:

- icon
- label
- accent color

The speed badge should not move or resize between templates.

## Information Hierarchy

Highest priority:

- card name
- speed badge
- rules text

Secondary priority:

- frame family
- subtype or type label
- artwork

Lowest priority:

- flavor text
- set metadata

## Typography

Recommended hierarchy:

- card name: expressive display serif or stylized title face
- rules text: highly readable body font
- type/meta text: neutral support font

Guidelines:

- never use the display face in the rules box
- support two-line names before scaling down
- keep rules text left aligned
- prefer readable line spacing over squeezing extra copy

## Rules Text Conventions

Rules text should be authored to fit a compact effect box.

Guidelines:

- keep one effect per line where practical
- allow bold or tinted keyword lead-ins later
- reserve enough vertical rhythm for `2` to `5` short lines
- avoid fully justified text

## Renderer Contract

The renderer should combine:

- one template family
- one speed badge
- card data for title, type, art, rules, flavor, and metadata

Template family should come from card alignment:

- `magical_girl` alignment -> `magical_girl` frame
- `baddie` alignment -> `baddie` frame
- `neutral` alignment -> `neutral` frame

Speed should come from the card's timing:

- `daily_life`
- `reaction`
- `encounter`

Examples:

- a Magical Girl encounter card uses the `magical_girl` frame plus the `encounter` badge
- a Baddie reaction card uses the `baddie` frame plus the `reaction` badge
- a neutral shared tactic uses the `neutral` frame

## Asset Layer Order

Recommended layer order:

1. background texture
2. outer frame
3. ornament overlay
4. art mask
5. art
6. name bar
7. speed badge base
8. speed icon and label
9. type row
10. rules panel
11. flavor panel
12. footer metadata
13. optional gloss or noise overlay

## Asset Naming

Use stable predictable names:

- `card_template_magical_girl.png`
- `card_template_baddie.png`
- `card_template_neutral.png`
- `badge_daily_life.png`
- `badge_reaction.png`
- `badge_encounter.png`

Current procedural exports live under `assets/generated/cards/`.
The source-of-truth layout and palette metadata stays in `assets/data/card_visuals.json`,
and `tools/generate_card_assets.py` regenerates the shipped PNG files from that data.
The procedural export now also includes layered pieces for each frame family:

- background
- frame
- ornament
- textbox
- art mask
- gloss

Each speed badge also exports:

- badge base
- badge icon
- combined labeled badge

If exported as layered pieces:

- `template_magical_girl_frame.png`
- `template_magical_girl_ornament.png`
- `template_magical_girl_textbox.png`

## Non-Negotiable Constraints

- do not move core layout zones between template families
- do not make speed require a full template swap
- do not hide critical rules information behind decoration
- do not let ornament reduce text readability
- keep the system data-driven so art assets and render code can evolve separately
