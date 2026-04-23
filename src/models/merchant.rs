use serde::Deserialize;

/// Information about the merchant application.
#[derive(Deserialize, Debug, Clone)]
pub struct InfoMerchant {
    pub uuid: String,
    pub name: String,
    pub url: String,
    pub desc: String,
    pub callback: String,
    pub success_url: String,
    pub cancel_url: String,
    pub logo: String,
    pub active: bool,
    pub enabled: bool,
    pub card: bool,
    pub app_photo_url: String,
}

/// Merchant balance information.
#[derive(Deserialize, Debug, Clone)]
pub struct BalanceMerchant {
    pub balance: f64,
}
