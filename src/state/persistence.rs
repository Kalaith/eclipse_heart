//! Local save loading and writing.

use std::path::PathBuf;

use thiserror::Error;

use super::{CampaignSaveBundle, CollectionSave, DecksSave, ProfileSave, SettingsSave};
use macroquad_toolkit::persistence::SaveRoot;

const PROFILE_FILE: &str = "profile.json";
const COLLECTION_FILE: &str = "collection.json";
const DECKS_FILE: &str = "decks.json";
const CAMPAIGNS_FILE: &str = "campaigns.json";
const SETTINGS_FILE: &str = "settings.json";

#[derive(Clone, Debug, Default)]
pub struct PersistenceBundle {
    pub profile: ProfileSave,
    pub collection: CollectionSave,
    pub decks: DecksSave,
    pub campaigns: CampaignSaveBundle,
    pub settings: SettingsSave,
}

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("failed to access save path: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse save json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to access save bundle: {0}")]
    Storage(String),
}

#[derive(Clone, Debug)]
pub struct PersistenceManager {
    root: SaveRoot,
}

impl PersistenceManager {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: SaveRoot::new("eclipse_heart", root.into()),
        }
    }

    pub fn default_local() -> Self {
        Self::new("save")
    }

    pub fn load_all(&self) -> Result<PersistenceBundle, PersistenceError> {
        Ok(PersistenceBundle {
            profile: self
                .root
                .load_json_or_default(PROFILE_FILE)
                .map_err(PersistenceError::Storage)?,
            collection: self
                .root
                .load_json_or_default(COLLECTION_FILE)
                .map_err(PersistenceError::Storage)?,
            decks: self
                .root
                .load_json_or_default(DECKS_FILE)
                .map_err(PersistenceError::Storage)?,
            campaigns: self
                .root
                .load_json_or_default(CAMPAIGNS_FILE)
                .map_err(PersistenceError::Storage)?,
            settings: self
                .root
                .load_json_or_default(SETTINGS_FILE)
                .map_err(PersistenceError::Storage)?,
        })
    }

    pub fn save_all(&self, bundle: &PersistenceBundle) -> Result<(), PersistenceError> {
        self.root
            .save_json(PROFILE_FILE, &bundle.profile)
            .map_err(PersistenceError::Storage)?;
        self.root
            .save_json(COLLECTION_FILE, &bundle.collection)
            .map_err(PersistenceError::Storage)?;
        self.root
            .save_json(DECKS_FILE, &bundle.decks)
            .map_err(PersistenceError::Storage)?;
        self.root
            .save_json(CAMPAIGNS_FILE, &bundle.campaigns)
            .map_err(PersistenceError::Storage)?;
        self.root
            .save_json(SETTINGS_FILE, &bundle.settings)
            .map_err(PersistenceError::Storage)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::state::CollectionCardKind;

    use super::{PersistenceBundle, PersistenceManager};

    fn temp_save_dir() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!("eclipse_heart_save_test_{unique}"))
    }

    #[test]
    fn missing_files_load_defaults() {
        let root = temp_save_dir();
        let manager = PersistenceManager::new(&root);

        let bundle = manager.load_all().expect("loads defaults");

        assert_eq!(bundle.profile.version, 1);
        assert!(bundle.collection.owned_magical_girls.is_empty());
        assert!(bundle.decks.support_decks.is_empty());
        assert!(bundle.campaigns.runs.is_empty());
        assert_eq!(bundle.settings.window_width, 2560);
        assert!(bundle.settings.fullscreen);

        std::fs::remove_dir_all(root).expect("cleanup temp dir");
    }

    #[test]
    fn save_round_trip_preserves_values() {
        let root = temp_save_dir();
        let manager = PersistenceManager::new(&root);
        let mut bundle = PersistenceBundle::default();
        bundle.profile.player_name = "Yuki".to_owned();
        bundle.profile.total_matches_played = 7;
        bundle.profile.total_wins = 5;
        bundle
            .collection
            .add_owned(CollectionCardKind::MagicalGirl, "yuki", 1);
        bundle.collection.add_owned(
            CollectionCardKind::StoryCard,
            "quiet_lunch_on_the_rooftop",
            2,
        );
        bundle.decks.roster_presets = vec!["starter_a".to_owned()];
        bundle.campaigns.runs = Vec::new();
        bundle.settings.fullscreen = true;

        manager.save_all(&bundle).expect("save bundle");
        let loaded = manager.load_all().expect("reload bundle");

        assert_eq!(loaded.profile.player_name, "Yuki");
        assert_eq!(loaded.profile.total_wins, 5);
        assert_eq!(
            loaded
                .collection
                .owned_count(CollectionCardKind::MagicalGirl, "yuki"),
            1
        );
        assert_eq!(
            loaded
                .collection
                .owned_count(CollectionCardKind::StoryCard, "quiet_lunch_on_the_rooftop"),
            2
        );
        assert_eq!(loaded.decks.roster_presets, vec!["starter_a"]);
        assert!(loaded.campaigns.runs.is_empty());
        assert!(loaded.settings.fullscreen);

        std::fs::remove_dir_all(root).expect("cleanup temp dir");
    }
}
