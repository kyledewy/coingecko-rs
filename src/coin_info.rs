use std::collections::HashMap;

/// Information about a coin. INCOMPLETE
#[derive(Debug, Deserialize)]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: Option<u64>,
    pub block_time_in_minutes: u64,
    pub hashing_algorithm: Option<String>,
    pub categories: Vec<String>,
    pub public_notice: Option<String>,
    pub localization: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub links: Links,
    pub image: ImageLinks,
}

#[derive(Debug, Deserialize)]
pub struct ImageLinks {
    pub thumb: String,
    pub small: String,
    pub large: String,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub homepage: Vec<String>,
    pub blockchain_site: Vec<String>,
    pub official_forum_url: Vec<String>,
    pub chat_url: Vec<String>,
    pub announcement_url: Vec<String>,
    pub twitter_screen_name: Option<String>,
    pub facebook_username: Option<String>,
    pub bitcointalk_thread_identifier: Option<u64>,
    pub telegram_channel_identifier: Option<String>,
    pub subreddit_url: Option<String>,
    pub repos_url: HashMap<String, Vec<String>>,
}
