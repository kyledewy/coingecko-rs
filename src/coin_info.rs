use std::collections::HashMap;

/// Information about a coin. INCOMPLETE
#[derive(Debug, Deserialize)]
pub struct CoinInfo {
    id: String,
    symbol: String,
    name: String,
    asset_platform_id: Option<u64>,
    block_time_in_minutes: u64,
    hashing_algorithm: Option<String>,
    categories: Vec<String>,
    public_notice: Option<String>,
    localization: HashMap<String, String>,
    description: HashMap<String, String>,
    links: Links,
    image: ImageLinks,
}

#[derive(Debug, Deserialize)]
pub struct ImageLinks {
    thumb: String,
    small: String,
    large: String,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    homepage: Vec<String>,
    blockchain_site: Vec<String>,
    official_forum_url: Vec<String>,
    chat_url: Vec<String>,
    announcement_url: Vec<String>,
    twitter_screen_name: Option<String>,
    facebook_username: Option<String>,
    bitcointalk_thread_identifier: Option<u64>,
    telegram_channel_identifier: Option<String>,
    subreddit_url: Option<String>,
    repos_url: HashMap<String, Vec<String>>,
}
