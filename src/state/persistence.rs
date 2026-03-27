//! Local save loading and writing.

use std::fs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

use super::{CampaignSaveBundle, CollectionSave, DecksSave, ProfileSave, SettingsSave};

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
}

#[derive(Clone, Debug)]
pub struct PersistenceManager {
    root: PathBuf,
}

impl PersistenceManager {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn default_local() -> Self {
        Self::new("save")
    }

    pub fn load_all(&self) -> Result<PersistenceBundle, PersistenceError> {
        fs::create_dir_all(&self.root)?;

        Ok(PersistenceBundle {
            profile: load_or_default(self.root.join(PROFILE_FILE))?,
            collection: load_or_default(self.root.join(COLLECTION_FILE))?,
            decks: load_or_default(self.root.join(DECKS_FILE))?,
            campaigns: load_or_default(self.root.join(CAMPAIGNS_FILE))?,
            settings: load_or_default(self.root.join(SETTINGS_FILE))?,
        })
    }

    pub fn save_all(&self, bundle: &PersistenceBundle) -> Result<(), PersistenceError> {
        fs::create_dir_all(&self.root)?;

        save_json(self.root.join(PROFILE_FILE), &bundle.profile)?;
        save_json(self.root.join(COLLECTION_FILE), &bundle.collection)?;
        save_json(self.root.join(DECKS_FILE), &bundle.decks)?;
        save_json(self.root.join(CAMPAIGNS_FILE), &bundle.campaigns)?;
        save_json(self.root.join(SETTINGS_FILE), &bundle.settings)?;
        Ok(())
    }
}

fn load_or_default<T>(path: PathBuf) -> Result<T, PersistenceError>
where
    T: DeserializeOwned + Default,
{
    if !path.exists() {
        return Ok(T::default());
    }

    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

fn save_json<T>(path: PathBuf, value: &T) -> Result<(), PersistenceError>
where
    T: Serialize,
{
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, text)?;
    Ok(())
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
