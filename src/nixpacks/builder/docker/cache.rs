use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::nixpacks::cache::{Cache, CacheKey};

pub struct DockerCache {
    cache_location: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedDockerImage {
    pub sha256: String,
    pub name: String,
}

impl DockerCache {
    pub fn new(cache_location: &str) -> Self {
        // Ensure that the cache_location exists
        let loc = PathBuf::from(cache_location);
        if !loc.is_dir() {
            fs::create_dir(loc.clone()).unwrap();
        }

        DockerCache {
            cache_location: loc,
        }
    }

    fn get_cache_value(&self, cache_key: &CacheKey) -> Result<Option<CachedDockerImage>> {
        let cache_path = self.get_cache_path(cache_key);
        if cache_path.is_file() {
            let cache_contents = fs::read_to_string(cache_path)?;
            let cache_value = serde_json::from_str::<CachedDockerImage>(cache_contents.as_str())?;
            Ok(Some(cache_value))
        } else {
            Ok(None)
        }
    }

    fn get_cache_path(&self, cache_key: &CacheKey) -> PathBuf {
        self.cache_location
            .clone()
            .join(format!("{cache_key}.json"))
    }
}

impl Cache<CachedDockerImage> for DockerCache {
    fn get_cached_value(&self, cache_key: &CacheKey) -> Result<Option<CachedDockerImage>> {
        // Look up /{cache_location}/{cache_key}
        match self.get_cache_value(cache_key)? {
            None => Ok(None),
            Some(cache_value) => Ok(Some(cache_value)),
        }
    }

    fn save_cached_value(&self, cache_key: CacheKey, value: CachedDockerImage) -> Result<()> {
        // Save to /{cache_location}/{cache_key}
        let cache_path = self.get_cache_path(&cache_key);
        fs::write(cache_path, serde_json::to_string_pretty(&value)?)?;

        Ok(())
    }
}