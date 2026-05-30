use macroquad::rand::gen_range;

use crate::data::GameContent;
use crate::state::{BoosterCardGrant, CollectionCardKind};

pub(super) fn open_booster(content: &GameContent) -> Vec<BoosterCardGrant> {
    let mut pool = Vec::new();

    for entry in &content.magical_girls {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::MagicalGirl,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    for entry in &content.baddies {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::Baddie,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    for entry in &content.story_cards {
        pool.push(BoosterCardGrant {
            kind: CollectionCardKind::StoryCard,
            id: entry.id.clone(),
            name: entry.name.clone(),
        });
    }

    let mut results = Vec::new();
    for _ in 0..10 {
        let index = gen_range(0, pool.len() as i32) as usize;
        results.push(pool[index].clone());
    }
    results
}
