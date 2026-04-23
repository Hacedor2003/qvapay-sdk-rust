use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::merchant::*;

impl QvaPayClient {
    /// Obtiene el balance del propietario de la aplicación.
    pub async fn get_balance(&self) -> Result<BalanceMerchant, SdkError> {
        self.get("/v2/balance").await
    }

    /// Obtiene la información de la aplicación registrada.
    pub async fn get_info(&self) -> Result<InfoMerchant, SdkError> {
        self.get("/info").await
    }
}
