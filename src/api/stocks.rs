use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::stocks::*;

impl QvaPayClient {
    /// Obtiene la lista de acciones disponibles.
    pub async fn get_stocks(&self) -> Result<Vec<Stock>, SdkError> {
        self.get("/stocks").await
    }

    /// Compra una cantidad de una acción.
    pub async fn buy_stock(&self, symbol: &str, amount: f64) -> Result<TradeResponse, SdkError> {
        let path = format!("/stocks/{}/buy", symbol);
        self.post(&path, &BuyStockRequest { amount }).await
    }

    /// Vende una cantidad de una acción.
    pub async fn sell_stock(&self, symbol: &str, quantity: f64) -> Result<TradeResponse, SdkError> {
        let path = format!("/stocks/{}/sell", symbol);
        self.post(&path, &SellStockRequest { quantity }).await
    }
}
