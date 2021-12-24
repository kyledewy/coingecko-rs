use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: HashMap<String, Option<Decimal>>,
    pub market_cap: HashMap<String, Option<Decimal>>,
    pub total_volume: HashMap<String, Option<Decimal>>,
}

#[derive(Deserialize, Debug)]
pub struct CommunityData {
        pub facebook_likes: Option<u64>,
        pub twitter_followers: Option<u64>,
        pub reddit_average_posts_48h: Option<Decimal>,
        pub reddit_average_comments_48h: Option<Decimal> ,
        pub reddit_subscribers: Option<u64>,
        pub reddit_accounts_active_48h: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct DeveloperData {
        pub forks: Option<u64>,
        pub stars: Option<u64>,
        pub subscribers: Option<u64>,
        pub total_issues: Option<u64>,
        pub closed_issues: Option<u64>,
        pub pull_requests_merged: Option<u64>,
        pub pull_request_contributors: Option<u64>,
        pub code_additions_deletions_4_weeks: HashMap<String, Option<i64>>,
        pub commit_count_4_weeks: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct HistoricPrice {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: HashMap<String, String>,
    pub market_data: MarketData,
    pub community_data: CommunityData,
    pub developer_data: DeveloperData,
    pub public_interest_stats: HashMap<String, Option<u64>>,
}

#[derive(Default, Setters)]
pub struct HistoricPriceReq {
    /// id of coin
    #[setters(skip)]
    pub id: String,

    /// date (dd-mm-yyyy ie. 12-03-2019)
    #[setters(skip)]
    pub date: String,

    /// Set to false to exclude localized languages in response
    #[setters(skip)]
    pub localization: bool,
}

impl HistoricPriceReq {
    pub fn new(id: String, date: String) -> Self {
        Self {
            id,
            date,
            localization: false,
        }
    }

    pub fn query(&self) -> String {
        fomat!(
            "id=" (self.id)
            "&date=" (self.date)
            "&localization=" (self.localization)
        )
    }
}
