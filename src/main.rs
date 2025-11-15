use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct CoinPriceResponse {
    coin: String,
    price: f32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/fetch-prices/:coin", get(fetch_prices));

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn fetch_prices(Path(coin): Path<String>)
    -> Json<CoinPriceResponse>
{
    let url = format!("https://api.coingecko.com/api/v3/coins/{}", coin);

    // Fetch RAW JSON
    let raw: Value = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    // Extract USD price
    let price = raw["market_data"]["current_price"]["usd"]
        .as_f64()
        .unwrap_or(0.0);

    Json(CoinPriceResponse {
        coin,
        price: price as f32,
    })
}
