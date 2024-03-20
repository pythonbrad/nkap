use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Based on the API data refresh time.
const LIFETIME: u64 = 3600;

/// Wrapper for the cache value.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Cache {
    timestamp: u64,
    data: String,
}

impl Cache {
    /// Initializes a cache value.
    ///
    /// **Note**: The cached data should implement [`Serialize`](serde::Serialize)?
    pub fn new(data: &impl Serialize) -> Result<Self> {
        let data = serde_json::to_string(&data)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Self { timestamp, data })
    }

    /// Dump the cache value.
    pub fn dump(&self, filename: &str) -> Result<()> {
        let data = serde_json::to_string(self)?;

        fs::write(filename, data).with_context(|| "Failed to write the cache")
    }

    /// Load the cache.
    ///
    /// **Note**: An error will be returned in case of:
    /// - cache not found
    /// - unexpected error
    pub fn load(filename: &str) -> Result<Self> {
        let data = fs::read(filename).with_context(|| "Failed to read the cache")?;
        let data = String::from_utf8_lossy(&data).to_string();

        Ok(serde_json::from_str(&data)?)
    }

    /// Unwrap the cached data.
    /// An error can be returned in case of cache expired.
    /// **Note**: The target type should implement [`Deserialize`](serde::Deserialize).
    pub fn unwrap<T>(&self) -> Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        if self.is_expired() {
            return Err(anyhow!("The cache has expired."));
        }

        Ok(serde_json::from_str(&self.data)?)
    }

    /// Whether the cache is expired.
    ///
    /// The expiration period is defined by [`LIFETIME`](self::LIFETIME).
    fn is_expired(&self) -> bool {
        let now = SystemTime::now();
        let old = UNIX_EPOCH + Duration::from_secs(self.timestamp);
        let diff = now.duration_since(old).unwrap();

        diff.as_secs() > LIFETIME
    }
}

#[test]
fn test_cache() {
    use crate::Cache;
    use std::env;

    let filename = env::temp_dir().join(".cache_test");
    let filename = filename.to_str().unwrap();
    let data = String::from("hello");

    let cache = Cache::new(&data).unwrap();
    cache.dump(filename).unwrap();
    let cache = Cache::load(filename).unwrap();

    assert_eq!(cache.unwrap::<String>().ok(), Some("hello".to_string()));
    assert!(!cache.is_expired());
}
