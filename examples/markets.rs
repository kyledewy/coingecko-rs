use coingecko::{CoinsMarketsReq, MarketsReqOrder};

pub fn main() {
    smol::block_on(async {
        let http = isahc::HttpClient::new().unwrap();

        let client = coingecko::Client::new(http);

        let req = CoinsMarketsReq::new("usd".into())
            .page(27)
            .per_page(250)
            .price_change_percentage("1h,200d".into())
            .order(MarketsReqOrder::MarketCapDesc)
            .sparkline();

        println!("{:#?}", client.coins_markets(req).await);
    })
}
