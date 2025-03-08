use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Rate limiter structure to maintain request timestamps
struct RateLimiter {
    requests: Vec<Instant>,
    max_requests: usize,
    time_window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, time_window_secs: u64) -> Self {
        Self {
            requests: Vec::with_capacity(max_requests),
            max_requests,
            time_window: Duration::from_secs(time_window_secs),
        }
    }
    
    fn can_make_request(&mut self) -> bool {
        let now = Instant::now();
        // Remove timestamps older than the time window
        self.requests.retain(|timestamp| now.duration_since(*timestamp) < self.time_window);
        
        // Check if we've reached the limit
        if self.requests.len() < self.max_requests {
            self.requests.push(now);
            true
        } else {
            false
        }
    }
    
    fn time_until_next_slot(&self) -> Duration {
        let now = Instant::now();
        if self.requests.len() < self.max_requests {
            return Duration::from_secs(0);
        }
        
        // Get the oldest request and calculate when it will expire
        if let Some(oldest) = self.requests.first() {
            let expiry = *oldest + self.time_window;
            if expiry > now {
                expiry - now
            } else {
                Duration::from_secs(0)
            }
        } else {
            Duration::from_secs(0)
        }
    }
}

pub struct Brave {
    client: Client,
    url: String,
    api_key: String,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SuggestResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub query: Query,
    pub results: Vec<Suggestion>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Query {
    pub original: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Suggestion {
    pub query: String,
}

impl Brave {
    pub fn new(url: String, api_key: String) -> Result<Self> {
        tracing::info!("Initializing Brave API client");
        let client = Client::new();
        // Configure rate limiter for 2 requests per second
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(1, 1)));
        
        Ok(Self {
            client,
            url: url.to_string(),
            api_key: api_key.to_string(),
            rate_limiter,
        })
    }

    pub async fn get_suggestions(&self, query: &str) -> Result<SuggestResponse> {
        tracing::info!("Fetching suggestions for query: {}", query);
        
        // Check rate limiting
        let can_proceed = {
            let mut limiter = self.rate_limiter.lock().unwrap();
            limiter.can_make_request()
        };
        
        if !can_proceed {
            // Calculate wait time and potentially wait if needed
            let wait_time = {
                self.rate_limiter.lock().unwrap().time_until_next_slot()
            };
            
            if wait_time > Duration::from_secs(0) {
                // Return rate limit error rather than waiting
                tracing::warn!("Rate limit exceeded for Brave API, need to wait {:?}", wait_time);
                return Err(anyhow::anyhow!("429"));
            }
            
            // Try again after rate limit window has passed
            let mut limiter = self.rate_limiter.lock().unwrap();
            if !limiter.can_make_request() {
                tracing::warn!("Rate limit still exceeded for Brave API");
                return Err(anyhow::anyhow!("429"));
            }
        }
        
        let response = self.client
            .get(&self.url)
            .header("X-Subscription-Token", &self.api_key)
            .query(&[
                ("q", query),
                ("country", "US"),
                ("rich", "false"),
            ])
            .send()
            .await?;

        // Log the raw response status for debugging
        let status = response.status();
        let response_body = response.text().await?;

        tracing::info!("Brave API response status: {}", status);

        if status == 429 {
            return Err(anyhow::anyhow!("429"));
        }

        // Parse the response body back to a Response to return
        let suggestions: SuggestResponse = serde_json::from_str(&response_body)?;
        tracing::info!("Successfully fetched {} suggestions", suggestions.results.len());
        Ok(suggestions)
    }
}