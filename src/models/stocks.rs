use serde::{Deserialize, Serialize};

/// Represents a stock/asset available in QvaPay.
#[derive(Deserialize, Debug, Clone)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub icon: String,
    #[serde(rename = "iconStyle")]
    pub icon_style: String,
    pub image: String,
    pub price: f64,
    pub change: f64,
    #[serde(rename = "changeDollar")]
    pub change_dollar: f64,
    pub volume: f64,
    pub timestamp: String,
}

/// Request to buy a stock.
#[derive(Serialize, Debug, Clone)]
pub struct BuyStockRequest {
    pub amount: f64,
}

/// Response from a trade operation (buy/sell).
#[derive(Deserialize, Debug, Clone)]
pub struct TradeResponse {
    pub trade_uuid: String,
    pub transaction_uuid: String,
    pub symbol: String,
    pub quantity: f64,
    pub effective_price: f64,
    pub market_price: f64,
    pub spread_percent: f64,
    pub fee: f64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Request to sell a stock.
#[derive(Serialize, Debug, Clone)]
pub struct SellStockRequest {
    pub quantity: f64,
}
