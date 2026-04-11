#!/usr/bin/env python3

import json
import math
import random
from pathlib import Path

from PIL import Image, ImageChops, ImageColor, ImageDraw, ImageFilter


ROOT = Path(__file__).resolve().parent.parent
ASSET_ROOT = ROOT / "assets"
DATA_ROOT = ASSET_ROOT / "data"
SPEC_PATH = DATA_ROOT / "card_visuals.json"
MAGICAL_GIRLS_PATH = DATA_ROOT / "magical_girls" / "prototype_set.json"
BADDIES_PATH = DATA_ROOT / "baddies" / "prototype_set.json"
STORY_CARDS_PATH = DATA_ROOT / "story_cards" / "prototype_set.json"
ART_CATALOG_PATH = DATA_ROOT / "art_catalog.json"


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def asset_path(relative_path):
    path = ASSET_ROOT / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    return path


def parse_color(value):
    return ImageColor.getrgb(value)


def mix(color_a, color_b, amount):
    return tuple(
        round(channel_a + (channel_b - channel_a) * amount)
        for channel_a, channel_b in zip(color_a, color_b)
    )


def tint(color, amount):
    target = (255, 255, 255) if amount >= 0 else (0, 0, 0)
    return mix(color, target, abs(amount))


def with_alpha(color, alpha):
    return color + (alpha,)


def zone_map(spec):
    return {zone["id"]: zone for zone in spec["zones"]}


def stable_seed(text):
    return sum((index + 1) * ord(char) for index, char in enumerate(text))


def create_vertical_gradient(size, top, bottom):
    width, height = size
    gradient = Image.new("RGB", size, top)
    pixels = gradient.load()
    for y in range(height):
        amount = y / max(height - 1, 1)
        row_color = mix(top, bottom, amount)
        for x in range(width):
            pixels[x, y] = row_color
    return gradient


def create_radial_glow(size, color, center, radius, strength):
    glow = Image.new("RGBA", size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(glow)
    for step in range(radius, 0, -12):
        alpha = round(255 * strength * (step / radius) ** 2)
        box = (
            center[0] - step,
            center[1] - step,
            center[0] + step,
            center[1] + step,
        )
        draw.ellipse(box, fill=color + (alpha,))
    return glow.filter(ImageFilter.GaussianBlur(radius=28))


def add_noise(base, seed, opacity):
    rng = random.Random(seed)
    width, height = base.size
    noise = Image.new("L", base.size)
    pixels = noise.load()
    for y in range(height):
        for x in range(width):
            pixels[x, y] = 108 + rng.randint(-40, 40)
    grain = Image.merge("RGBA", (noise, noise, noise, Image.new("L", base.size, opacity)))
    return ImageChops.overlay(base, grain)


def rounded_box(draw, rect, fill, outline, outline_width=3, radius=18):
    draw.rounded_rectangle(rect, radius=radius, fill=fill, outline=outline, width=outline_width)


def draw_star(draw, center, radius, fill, outline=None):
    points = []
    for index in range(10):
        angle = math.radians(-90 + index * 36)
        current_radius = radius if index % 2 == 0 else radius * 0.42
        points.append(
            (
                center[0] + math.cos(angle) * current_radius,
                center[1] + math.sin(angle) * current_radius,
            )
        )
    draw.polygon(points, fill=fill, outline=outline)


def draw_crescent(draw, center, radius, fill, cut_color, offset):
    draw.ellipse(
        (center[0] - radius, center[1] - radius, center[0] + radius, center[1] + radius),
        fill=fill,
    )
    draw.ellipse(
        (
            center[0] - radius + offset,
            center[1] - radius,
            center[0] + radius + offset,
            center[1] + radius,
        ),
        fill=cut_color,
    )


def draw_ribbon(draw, points, fill, width):
    draw.line(points, fill=fill, width=width, joint="curve")


def draw_thorns(draw, start_x, end_x, base_y, fill):
    step = 54
    points = []
    x = start_x
    while x <= end_x:
        points.extend(
            [
                (x, base_y),
                (x + step * 0.22, base_y - 22),
                (x + step * 0.5, base_y + 12),
            ]
        )
        x += step
    draw.line(points, fill=fill, width=4)


def draw_glass_shards(draw, bounds, fill):
    left, top, _, _ = bounds
    for shift in range(0, 5):
        offset = shift * 22
        draw.polygon(
            [
                (left + 30 + offset, top + 18),
                (left + 84 + offset, top + 2),
                (left + 64 + offset, top + 74),
            ],
            outline=fill,
        )


def draw_eye(draw, center, size, fill, accent):
    cx, cy = center
    width, height = size
    draw.ellipse((cx - width, cy - height, cx + width, cy + height), outline=fill, width=4)
    draw.ellipse(
        (cx - width * 0.25, cy - height * 0.25, cx + width * 0.25, cy + height * 0.25),
        fill=accent,
    )


def draw_grid(draw, bounds, fill):
    left, top, right, bottom = bounds
    spacing = 28
    for x in range(left, right + 1, spacing):
        draw.line((x, top, x, bottom), fill=fill, width=1)
    for y in range(top, bottom + 1, spacing):
        draw.line((left, y, right, y), fill=fill, width=1)


def draw_seal(draw, center, radius, fill, accent):
    cx, cy = center
    draw.ellipse((cx - radius, cy - radius, cx + radius, cy + radius), outline=fill, width=4)
    for angle in range(0, 360, 45):
        radians = math.radians(angle)
        x = cx + math.cos(radians) * radius
        y = cy + math.sin(radians) * radius
        draw.line((cx, cy, x, y), fill=accent, width=2)


def family_style(family_id):
    return {
        "magical_girl": {"line_radius": 24, "panel_radius": 22, "ornament_alpha": 136},
        "baddie": {"line_radius": 10, "panel_radius": 12, "ornament_alpha": 148},
        "neutral": {"line_radius": 14, "panel_radius": 16, "ornament_alpha": 122},
    }[family_id]


def card_template_layers(spec, family):
    canvas = spec["canvas"]
    size = (canvas["width"], canvas["height"])
    palette = [parse_color(value) for value in family["palette"]]
    zones = zone_map(spec)
    family_id = family["id"]
    seed = stable_seed(family_id)
    style = family_style(family_id)

    background = create_vertical_gradient(size, tint(palette[0], 0.04), tint(palette[1], -0.36)).convert(
        "RGBA"
    )
    background.alpha_composite(create_radial_glow(size, palette[2], (110, 120), 210, 0.42))
    background.alpha_composite(create_radial_glow(size, palette[3], (620, 920), 240, 0.24))
    background = add_noise(background, seed, 34)

    frame = Image.new("RGBA", size, (0, 0, 0, 0))
    frame_draw = ImageDraw.Draw(frame)
    safe_margin = canvas["safe_margin"]
    outer = (safe_margin, safe_margin, size[0] - safe_margin, size[1] - safe_margin)
    rounded_box(
        frame_draw,
        outer,
        fill=(0, 0, 0, 0),
        outline=with_alpha(tint(palette[2], -0.15), 255),
        outline_width=6,
        radius=style["line_radius"],
    )
    rounded_box(
        frame_draw,
        (outer[0] + 10, outer[1] + 10, outer[2] - 10, outer[3] - 10),
        fill=(0, 0, 0, 0),
        outline=with_alpha(tint(palette[3], -0.1), 235),
        outline_width=2,
        radius=max(style["line_radius"] - 4, 8),
    )

    art_zone = zones["art_frame"]
    art_bounds = (
        art_zone["x"],
        art_zone["y"],
        art_zone["x"] + art_zone["width"],
        art_zone["y"] + art_zone["height"],
    )
    rounded_box(
        frame_draw,
        art_bounds,
        fill=(0, 0, 0, 0),
        outline=with_alpha(tint(palette[2], -0.22), 255),
        outline_width=4,
        radius=style["panel_radius"],
    )
    badge_zone = zones["speed_badge"]
    frame_draw.rounded_rectangle(
        (
            badge_zone["x"],
            badge_zone["y"],
            badge_zone["x"] + badge_zone["width"],
            badge_zone["y"] + badge_zone["height"],
        ),
        radius=style["panel_radius"],
        outline=with_alpha(tint(palette[2], -0.18), 255),
        width=4,
    )

    textboxes = Image.new("RGBA", size, (0, 0, 0, 0))
    text_draw = ImageDraw.Draw(textboxes)
    for zone_name in ("name_bar", "type_row", "rules_box", "flavor_box", "footer_meta"):
        zone = zones[zone_name]
        zone_bounds = (zone["x"], zone["y"], zone["x"] + zone["width"], zone["y"] + zone["height"])
        rounded_box(
            text_draw,
            zone_bounds,
            fill=with_alpha(tint(palette[0], 0.03), 232),
            outline=with_alpha(tint(palette[3], -0.14), 240),
            outline_width=3,
            radius=style["panel_radius"],
        )

    ornament = Image.new("RGBA", size, (0, 0, 0, 0))
    ornament_draw = ImageDraw.Draw(ornament)
    pale = with_alpha(tint(palette[0], 0.12), style["ornament_alpha"])
    bright = with_alpha(tint(palette[3], 0.04), style["ornament_alpha"] + 20)

    if family_id == "magical_girl":
        draw_crescent(ornament_draw, (126, 214), 58, pale, (0, 0, 0, 0), 24)
        draw_star(ornament_draw, (604, 188), 34, bright)
        draw_star(ornament_draw, (156, 834), 20, pale)
        draw_star(ornament_draw, (640, 846), 16, bright)
        draw_ribbon(ornament_draw, [(68, 608), (178, 582), (312, 610), (472, 596), (682, 620)], bright, 7)
        draw_ribbon(ornament_draw, [(82, 960), (222, 930), (372, 944), (544, 920), (690, 936)], pale, 5)
    elif family_id == "baddie":
        draw_thorns(ornament_draw, 76, 678, 124, bright)
        draw_thorns(ornament_draw, 84, 678, 952, pale)
        draw_glass_shards(ornament_draw, (88, 154, 286, 248), pale)
        draw_glass_shards(ornament_draw, (480, 418, 690, 518), bright)
        draw_eye(ornament_draw, (598, 212), (42, 24), bright, with_alpha(tint(palette[2], 0.18), 255))
        ornament_draw.rectangle((72, 544, 678, 556), fill=with_alpha(tint(palette[2], -0.15), 120))
    else:
        draw_grid(ornament_draw, (76, 152, 674, 524), pale)
        draw_grid(ornament_draw, (76, 628, 674, 900), with_alpha(tint(palette[1], 0.18), 56))
        draw_seal(ornament_draw, (126, 218), 44, bright, pale)
        draw_seal(ornament_draw, (624, 938), 36, pale, bright)
        ornament_draw.line((92, 118, 662, 118), fill=bright, width=3)
        ornament_draw.line((92, 976, 662, 976), fill=bright, width=3)

    art_mask = Image.new("RGBA", size, (0, 0, 0, 0))
    art_mask_draw = ImageDraw.Draw(art_mask)
    art_mask_draw.rounded_rectangle(art_bounds, radius=style["panel_radius"], fill=(255, 255, 255, 255))

    gloss = Image.new("RGBA", size, (0, 0, 0, 0))
    gloss_draw = ImageDraw.Draw(gloss)
    gloss_draw.polygon([(52, 52), (276, 52), (508, 998), (324, 998)], fill=(255, 255, 255, 28))
    gloss = gloss.filter(ImageFilter.GaussianBlur(radius=18))

    flat = background.copy()
    flat.alpha_composite(frame)
    flat.alpha_composite(ornament)
    flat.alpha_composite(textboxes)
    flat.alpha_composite(gloss)

    return {
        "background": background,
        "frame": frame,
        "ornament": ornament,
        "textbox": textboxes,
        "art_mask": art_mask,
        "gloss": gloss,
        "flat": flat,
    }


def badge_icon_kind(badge_id):
    return {"daily_life": "sun", "reaction": "bolt", "encounter": "burst"}[badge_id]


def badge_layers(zone, badge):
    size = (zone["width"], zone["height"])
    accent = parse_color(badge["accent_color"])
    seed = stable_seed(badge["id"])

    base = create_vertical_gradient(size, tint(accent, 0.35), tint(accent, -0.35)).convert("RGBA")
    base = add_noise(base, seed, 28)
    base_draw = ImageDraw.Draw(base)
    base_draw.rounded_rectangle((2, 2, size[0] - 3, size[1] - 3), radius=20, outline=(255, 255, 255, 230), width=3)

    icon = Image.new("RGBA", size, (0, 0, 0, 0))
    icon_draw = ImageDraw.Draw(icon)
    center_x = size[0] // 2
    icon_kind = badge_icon_kind(badge["id"])

    if icon_kind == "sun":
        icon_draw.ellipse((24, 15, 56, 47), fill=(255, 250, 233, 230))
        for angle in range(0, 360, 45):
            radians = math.radians(angle)
            inner = (center_x + math.cos(radians) * 22, 31 + math.sin(radians) * 22)
            outer = (center_x + math.cos(radians) * 30, 31 + math.sin(radians) * 30)
            icon_draw.line((*inner, *outer), fill=(255, 250, 233, 220), width=3)
    elif icon_kind == "bolt":
        icon_draw.polygon(
            [(36, 12), (54, 12), (46, 30), (58, 30), (28, 58), (38, 36), (24, 36)],
            fill=(255, 244, 232, 232),
        )
    else:
        draw_star(icon_draw, (center_x, 30), 20, (255, 243, 235, 236))
        icon_draw.ellipse((28, 18, 52, 42), outline=(255, 255, 255, 180), width=2)

    flat = base.copy()
    flat.alpha_composite(icon)
    text_draw = ImageDraw.Draw(flat)
    text = badge["short_label"].upper()
    text_width = text_draw.textlength(text)
    text_draw.text(((size[0] - text_width) / 2, 54), text, fill=(255, 255, 255, 238))

    return {"base": base, "icon": icon, "flat": flat}


def draw_portrait_shape(draw, role, accent, highlight):
    if role == "magical_girl":
        draw.ellipse((156, 84, 356, 284), fill=highlight)
        draw.polygon([(132, 470), (256, 258), (380, 470)], fill=with_alpha(tint(accent, -0.12), 228))
        draw.polygon([(194, 258), (256, 190), (318, 258), (340, 404), (172, 404)], fill=with_alpha(accent, 238))
        draw_crescent(draw, (350, 126), 34, with_alpha(tint(accent, 0.45), 220), (0, 0, 0, 0), 14)
        draw_star(draw, (102, 156), 18, with_alpha(tint(accent, 0.6), 200))
    else:
        draw.ellipse((156, 86, 356, 286), fill=with_alpha(tint(accent, -0.28), 238))
        draw.polygon([(128, 468), (188, 272), (324, 212), (386, 468)], fill=with_alpha(accent, 236))
        draw.polygon([(184, 136), (258, 62), (334, 138), (308, 198), (210, 198)], fill=with_alpha(tint(accent, -0.16), 248))
        draw_thorns(draw, 92, 414, 426, with_alpha(tint(accent, 0.35), 176))
        draw_eye(draw, (366, 144), (34, 18), with_alpha(tint(accent, 0.52), 228), with_alpha((255, 255, 255), 188))


def render_character_portrait(character, role):
    seed = stable_seed(character["id"])
    rng = random.Random(seed)
    palette = (
        ((255, 235, 242), (104, 190, 255), (255, 110, 162))
        if role == "magical_girl"
        else ((33, 24, 40), (116, 34, 70), (29, 170, 164))
    )
    base = create_vertical_gradient((512, 512), palette[0], tint(palette[1], -0.3)).convert("RGBA")
    base.alpha_composite(create_radial_glow((512, 512), palette[2], (128, 116), 156, 0.48))
    base.alpha_composite(create_radial_glow((512, 512), tint(palette[1], 0.22), (392, 402), 178, 0.26))
    base = add_noise(base, seed, 26)
    draw = ImageDraw.Draw(base)
    draw.rounded_rectangle((18, 18, 494, 494), radius=28, outline=with_alpha(tint(palette[2], -0.14), 255), width=6)

    accent = mix(palette[1], palette[2], 0.5 if role == "magical_girl" else 0.35)
    highlight = with_alpha(tint(palette[0], 0.18), 224)
    draw_portrait_shape(draw, role, accent, highlight)

    for _ in range(8):
        x = rng.randint(42, 470)
        y = rng.randint(42, 470)
        size = rng.randint(10, 24)
        if role == "magical_girl":
            draw_star(draw, (x, y), size, with_alpha(tint(accent, 0.35), 90))
        else:
            draw_glass_shards(draw, (x - 20, y - 16, x + 20, y + 16), with_alpha(tint(accent, 0.22), 90))
    return base


def story_alignment_palette(alignment):
    return {
        "magical_girl": ((255, 244, 245), (126, 202, 255), (255, 128, 170)),
        "baddie": ((38, 25, 42), (139, 30, 63), (47, 176, 169)),
        "neutral": ((242, 232, 210), (154, 123, 73), (89, 97, 106)),
    }[alignment]


def render_story_card_art(card):
    seed = stable_seed(card["id"])
    alignment = card["alignment"]
    palette = story_alignment_palette(alignment)
    size = (646, 410)
    image = create_vertical_gradient(size, tint(palette[0], 0.04), tint(palette[1], -0.34)).convert("RGBA")
    image.alpha_composite(create_radial_glow(size, palette[2], (112, 92), 148, 0.44))
    image.alpha_composite(create_radial_glow(size, tint(palette[1], 0.24), (544, 318), 168, 0.24))
    image = add_noise(image, seed, 24)
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle((8, 8, 638, 402), radius=20, outline=(255, 255, 255, 180), width=3)

    speed = card["speed"]
    if speed == "daily_life":
        draw.ellipse((72, 84, 232, 244), fill=with_alpha(tint(palette[0], 0.2), 192))
        draw_ribbon(draw, [(48, 316), (164, 266), (294, 296), (428, 244), (598, 278)], with_alpha(palette[2], 156), 10)
    elif speed == "reaction":
        draw.polygon([(314, 50), (386, 50), (350, 154), (426, 154), (250, 332), (306, 204), (226, 204)], fill=with_alpha(tint(palette[2], 0.1), 196))
        draw.line((72, 324, 572, 112), fill=with_alpha(tint(palette[1], 0.28), 140), width=6)
    else:
        draw_star(draw, (186, 196), 72, with_alpha(tint(palette[2], 0.12), 188))
        draw.ellipse((338, 102, 556, 320), outline=with_alpha(tint(palette[0], 0.25), 174), width=8)

    card_type = card["card_type"]
    if card_type in {"bond", "rumor"}:
        draw_crescent(draw, (506, 118), 34, with_alpha(tint(palette[0], 0.24), 196), (0, 0, 0, 0), 12)
    elif card_type in {"scheme", "reaction"}:
        draw_eye(draw, (518, 102), (42, 20), with_alpha(tint(palette[2], 0.3), 180), with_alpha((255, 255, 255), 150))
    else:
        draw_seal(draw, (512, 104), 42, with_alpha(tint(palette[0], 0.18), 186), with_alpha(tint(palette[2], 0.06), 146))

    if alignment == "magical_girl":
        draw_ribbon(draw, [(42, 354), (152, 330), (278, 344), (398, 316), (602, 334)], with_alpha(tint(palette[2], 0.06), 116), 6)
    elif alignment == "baddie":
        draw_thorns(draw, 48, 606, 352, with_alpha(tint(palette[2], 0.22), 122))
    else:
        draw_grid(draw, (56, 58, 590, 346), with_alpha(tint(palette[1], 0.18), 44))

    image.alpha_composite(Image.new("RGBA", size, (255, 255, 255, 0)).filter(ImageFilter.GaussianBlur(radius=1)))
    return image


def render_ui_background(kind):
    size = (2560, 1440)
    palettes = {
        "menu": ((253, 240, 235), (121, 181, 255), (255, 127, 137)),
        "battle": ((30, 24, 36), (114, 42, 80), (35, 150, 156)),
        "campaign": ((246, 237, 214), (122, 96, 52), (195, 92, 78)),
    }
    top, middle, accent = palettes[kind]
    image = create_vertical_gradient(size, top, tint(middle, -0.34)).convert("RGBA")
    image.alpha_composite(create_radial_glow(size, accent, (420, 240), 360, 0.35))
    image.alpha_composite(create_radial_glow(size, tint(middle, 0.22), (2040, 1120), 420, 0.22))
    image = add_noise(image, stable_seed(kind), 20)
    draw = ImageDraw.Draw(image)
    if kind == "menu":
        for offset in range(0, 4):
            draw_ribbon(draw, [(120, 1180 - offset * 56), (640, 940 - offset * 36), (1240, 1060 - offset * 52), (2260, 900 - offset * 42)], with_alpha(accent, 64 - offset * 10), 20 - offset * 3)
            draw_star(draw, (430 + offset * 360, 210 + offset * 76), 40 - offset * 6, with_alpha(tint(accent, 0.24), 70))
    elif kind == "battle":
        for x in range(180, 2360, 220):
            draw_thorns(draw, x, x + 140, 1220, with_alpha(tint(accent, 0.16), 84))
        draw.line((220, 980, 2320, 520), fill=with_alpha(tint(middle, 0.28), 72), width=12)
        draw_eye(draw, (2050, 260), (92, 44), with_alpha(tint(accent, 0.22), 96), with_alpha((255, 255, 255), 82))
    else:
        draw_grid(draw, (180, 180, 2380, 1240), with_alpha(tint(middle, 0.14), 34))
        for center in ((360, 260), (2160, 1150), (1860, 340)):
            draw_seal(draw, center, 78, with_alpha(tint(accent, 0.14), 86), with_alpha(tint(middle, 0.22), 72))
    return image


def preview_sheet(images, title, thumb_size, columns, background=(18, 20, 28, 255)):
    rows = math.ceil(len(images) / columns)
    width = 48 + columns * (thumb_size[0] + 24)
    height = 92 + rows * (thumb_size[1] + 24)
    sheet = Image.new("RGBA", (width, height), background)
    draw = ImageDraw.Draw(sheet)
    draw.text((24, 20), title, fill=(244, 240, 232, 255))
    for index, image in enumerate(images):
        x = 24 + (index % columns) * (thumb_size[0] + 24)
        y = 60 + (index // columns) * (thumb_size[1] + 24)
        sheet.alpha_composite(image.resize(thumb_size), (x, y))
    return sheet


def render_all_card_assets(spec):
    template_paths = []
    layer_name_map = {
        "background_asset": "background",
        "frame_asset": "frame",
        "ornament_asset": "ornament",
        "textbox_asset": "textbox",
        "art_mask_asset": "art_mask",
        "gloss_asset": "gloss",
        "asset_name": "flat",
    }

    for family in spec["template_families"]:
        layers = card_template_layers(spec, family)
        for key, layer_name in layer_name_map.items():
            output_path = asset_path(family[key])
            layers[layer_name].save(output_path)
            if key == "asset_name":
                template_paths.append(output_path)

    zone = zone_map(spec)["speed_badge"]
    badge_paths = []
    badge_name_map = {"base_asset": "base", "icon_asset": "icon", "badge_asset": "flat"}
    for badge in spec["speed_badges"]:
        layers = badge_layers(zone, badge)
        for key, layer_name in badge_name_map.items():
            output_path = asset_path(badge[key])
            layers[layer_name].save(output_path)
            if key == "badge_asset":
                badge_paths.append(output_path)

    flat_templates = [Image.open(path).convert("RGBA").resize((300, 420)) for path in template_paths]
    flat_badges = [Image.open(path).convert("RGBA").resize((120, 120)) for path in badge_paths]
    sheet = Image.new("RGBA", (1024, 720), (18, 20, 28, 255))
    draw = ImageDraw.Draw(sheet)
    draw.text((36, 24), "Eclipse Heart Procedural Card Assets", fill=(244, 240, 232, 255))
    x = 36
    for card in flat_templates:
        sheet.alpha_composite(card, (x, 86))
        x += 326
    x = 206
    for badge in flat_badges:
        sheet.alpha_composite(badge, (x, 542))
        x += 204
    sheet.save(asset_path("generated/cards/card_asset_preview.png"))


def render_full_art_pack():
    magical_girls = load_json(MAGICAL_GIRLS_PATH)
    baddies = load_json(BADDIES_PATH)
    story_cards = load_json(STORY_CARDS_PATH)

    character_specs = []
    portrait_previews = []
    for role, roster in (("magical_girl", magical_girls), ("baddie", baddies)):
        for character in roster:
            relative_path = f"generated/portraits/{character['id']}.png"
            render_character_portrait(character, role).save(asset_path(relative_path))
            character_specs.append(
                {
                    "id": character["id"],
                    "role": role,
                    "asset_name": relative_path,
                    "accent_color": "#FF8CAB" if role == "magical_girl" else "#E53F8A",
                    "motifs": ["portrait", "base_form", "roster"],
                }
            )
            portrait_previews.append(Image.open(asset_path(relative_path)).convert("RGBA"))

    story_specs = []
    story_previews = []
    for card in story_cards:
        relative_path = f"generated/story_cards/{card['id']}.png"
        render_story_card_art(card).save(asset_path(relative_path))
        story_specs.append(
            {
                "id": card["id"],
                "asset_name": relative_path,
                "alignment": card["alignment"],
                "speed": card["speed"],
                "card_type": card["card_type"],
                "motifs": [card["alignment"], card["speed"], card["card_type"]],
            }
        )
        story_previews.append(Image.open(asset_path(relative_path)).convert("RGBA"))

    ui_specs = []
    ui_previews = []
    ui_metadata = {
        "menu": {"tone": "hopeful intro screen", "motifs": ["ribbon", "starfield", "dawn"]},
        "battle": {"tone": "dramatic versus backdrop", "motifs": ["thorn", "spotlight", "clash"]},
        "campaign": {"tone": "storybook progression map", "motifs": ["grid", "seal", "parchment"]},
    }
    for ui_id, meta in ui_metadata.items():
        relative_path = f"generated/ui/{ui_id}_background.png"
        render_ui_background(ui_id).save(asset_path(relative_path))
        ui_specs.append({"id": ui_id, "asset_name": relative_path, "tone": meta["tone"], "motifs": meta["motifs"]})
        ui_previews.append(Image.open(asset_path(relative_path)).convert("RGBA"))

    write_json(
        ART_CATALOG_PATH,
        {
            "ui_backgrounds": ui_specs,
            "character_portraits": character_specs,
            "story_card_art": story_specs,
        },
    )

    preview_sheet(portrait_previews, "Eclipse Heart Portrait Set", (180, 180), 5).save(
        asset_path("generated/portraits/portrait_preview.png")
    )
    preview_sheet(story_previews, "Eclipse Heart Story Card Art Set", (160, 102), 4).save(
        asset_path("generated/story_cards/story_card_art_preview.png")
    )
    preview_sheet(ui_previews, "Eclipse Heart UI Backdrops", (288, 162), 2).save(
        asset_path("generated/ui/ui_background_preview.png")
    )


def main():
    spec = load_json(SPEC_PATH)
    render_all_card_assets(spec)
    render_full_art_pack()


if __name__ == "__main__":
    main()
