use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlEntry {
    pub original_url: String,      // it has the original url
    pub code: String,              // it shorten code
    pub created_at: DateTime<Utc>, // time when the url created
}
