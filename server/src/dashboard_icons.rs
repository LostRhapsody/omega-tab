use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const METADATA_URL: &str =
    "https://raw.githubusercontent.com/homarr-labs/dashboard-icons/refs/heads/main/metadata.json";
const CDN_BASE: &str = "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons";
const CACHE_TTL: Duration = Duration::from_secs(60 * 60); // 1 hour
const MAX_RESULTS: usize = 20;

// Global cache for metadata
static METADATA_CACHE: LazyLock<RwLock<Option<CachedMetadata>>> =
    LazyLock::new(|| RwLock::new(None));

struct CachedMetadata {
    data: HashMap<String, IconMetadata>,
    fetched_at: Instant,
}

pub struct DashboardIcons {
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardIcon {
    pub name: String,
    pub png_url: String,
}

#[derive(Debug, Serialize)]
pub struct IconSearchResponse {
    pub icons: Vec<DashboardIcon>,
}

// Struct to deserialize the metadata.json format
#[derive(Debug, Clone, Deserialize)]
struct IconMetadata {
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    categories: Vec<String>,
}

impl DashboardIcons {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<IconSearchResponse> {
        tracing::info!("Searching dashboard icons for query: {}", query);

        let metadata = self.get_metadata().await?;
        let query_lower = query.to_lowercase();

        let icons: Vec<DashboardIcon> = metadata
            .into_iter()
            .filter(|(name, meta)| {
                // Match against icon name
                if name.to_lowercase().contains(&query_lower) {
                    return true;
                }
                // Match against aliases
                if meta
                    .aliases
                    .iter()
                    .any(|alias| alias.to_lowercase().contains(&query_lower))
                {
                    return true;
                }
                // Match against categories
                if meta
                    .categories
                    .iter()
                    .any(|cat| cat.to_lowercase().contains(&query_lower))
                {
                    return true;
                }
                false
            })
            .take(MAX_RESULTS)
            .map(|(name, _)| DashboardIcon {
                png_url: format!("{}/png/{}.png", CDN_BASE, name),
                name,
            })
            .collect();

        tracing::info!("Found {} icons for query: {}", icons.len(), query);
        Ok(IconSearchResponse { icons })
    }

    async fn get_metadata(&self) -> Result<HashMap<String, IconMetadata>> {
        // Check if we have valid cached data
        {
            let cache = METADATA_CACHE.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.fetched_at.elapsed() < CACHE_TTL {
                    tracing::debug!("Using cached dashboard icons metadata");
                    return Ok(cached.data.clone());
                }
            }
        }

        // Fetch fresh metadata
        tracing::info!("Fetching fresh dashboard icons metadata");
        let metadata = self.fetch_metadata().await?;

        // Update cache
        {
            let mut cache = METADATA_CACHE.write().await;
            *cache = Some(CachedMetadata {
                data: metadata.clone(),
                fetched_at: Instant::now(),
            });
        }

        Ok(metadata)
    }

    async fn fetch_metadata(&self) -> Result<HashMap<String, IconMetadata>> {
        let response = self.client.get(METADATA_URL).send().await?;

        if !response.status().is_success() {
            tracing::warn!(
                "Failed to fetch dashboard icons metadata, status: {}",
                response.status()
            );
            return Ok(HashMap::new());
        }

        let metadata: HashMap<String, IconMetadata> = response.json().await?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    // ensures we can fetch the dashboard icon meta data,
    // query it, receive icons, and not go over the limit
    async fn test_dashboard_search() {
        let dashboard_icons = DashboardIcons::new();
        let response = dashboard_icons.search("github").await.unwrap();
        println!("{:?}", response.icons);
        assert!(!response.icons.is_empty());
        assert!(response.icons.len() <= MAX_RESULTS);
    }
}
