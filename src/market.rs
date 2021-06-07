use std::fmt::Display;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: Option<Decimal>,
    pub market_cap: Option<Decimal>,
    pub market_cap_rank: Option<u64>,
    pub fully_diluted_valuation: Option<Decimal>,
    pub total_volume: Option<Decimal>,
    pub high_24h: Option<Decimal>,
    pub low_24h: Option<Decimal>,
    pub price_change_24h: Option<Decimal>,
    pub price_change_percentage_24h: Option<Decimal>,
    pub market_cap_change_24h: Option<Decimal>,
    pub market_cap_change_percentage_24h: Option<Decimal>,
    pub circulating_supply: Option<Decimal>,
    pub total_supply: Option<Decimal>,
    pub max_supply: Option<Decimal>,
    pub ath: Option<Decimal>,
    pub ath_change_percentage: Option<Decimal>,
    pub ath_date: Option<String>,
    pub atl: Option<Decimal>,
    pub atl_change_percentage: Option<Decimal>,
    pub atl_date: Option<String>,
    pub roi: Option<Roi>,
    pub last_updated: Option<String>,
    /// only available if specified in the request
    pub sparkline_in_7d: Option<SparklineIn7D>,

    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_14d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_1h_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_1y_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_200d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_24h_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_30d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_7d_in_currency: Option<Decimal>
}

#[derive(Debug, Deserialize)]
pub struct Roi {
    pub times: Option<Decimal>,
    pub currency: String,
    pub percentage: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct SparklineIn7D {
    pub price: Vec<Decimal>,
}

#[derive(Default, Setters)]
pub struct CoinsMarketsReq {
    /// target currency of market data
    #[setters(skip)]
    pub vs_currency: String,

    /// ids of coins, comma-separated
    #[setters(strip_option)]
    pub ids: Option<String>,

    /// filter by coin category
    #[setters(strip_option)]
    pub category: Option<String>,

    /// sort results by field
    #[setters(strip_option)]
    pub order: Option<MarketsReqOrder>,

    /// total results per page, valid values: 1...250
    #[setters(strip_option)]
    pub per_page: Option<u8>,

    #[setters(strip_option)]
    pub page: Option<u32>,

    #[setters(bool)]
    pub sparkline: bool,

    /// possible values (multiple, comma-separated): 1h, 24h, 7d, 14d, 30d, 200d, 1y
    #[setters(strip_option)]
    pub price_change_percentage: Option<String>,
}

impl CoinsMarketsReq {
    pub fn new(vs_currency: String) -> Self {
        Self {
            vs_currency,
            ..Default::default()
        }
    }

    pub fn query(&self) -> String {
        fomat!(
            "vs_currency=" (self.vs_currency)
            if let Some(ids) = &self.ids {
                "&ids=" (ids)
            }
            if let Some(category) = &self.category {
                "&category=" (category)
            }
            if let Some(order) = &self.order {
                "&order=" (order)
            }
            if let Some(per_page) = self.per_page {
                "&per_page=" (per_page)
            }
            if let Some(page) = self.page {
                "&page=" (page)
            }
            if (self.sparkline) {
                "&sparkline=true"
            }
            if let Some(price_change_percentage) = &self.price_change_percentage {
                "&price_change_percentage=" (price_change_percentage)
            }
        )
    }
}

pub enum MarketsReqOrder {
    GeckoDesc,
    GeckoAsc,
    MarketCapAsc,
    MarketCapDesc,
    VolumeAsc,
    VolumeDesc,
    IdAsc,
    IdDesc,
}

impl Display for MarketsReqOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MarketsReqOrder::GeckoDesc => "gecko_desc",
            MarketsReqOrder::GeckoAsc => "gecko_asc",
            MarketsReqOrder::MarketCapAsc => "market_cap_asc",
            MarketsReqOrder::MarketCapDesc => "market_cap_desc",
            MarketsReqOrder::VolumeAsc => "volume_asc",
            MarketsReqOrder::VolumeDesc => "volume_desc",
            MarketsReqOrder::IdAsc => "id_asc",
            MarketsReqOrder::IdDesc => "id_desc",
        })
    }
}