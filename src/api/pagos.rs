use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::pago::*;

impl QvaPayClient {
    /// Crea una nueva factura (pago) en QvaPay.
    ///
    /// # Ejemplo
    /// ```no_run
    /// # use qvapay_sdk::{QvaPayClient, Environment, models::pago::CreateInvoiceRequest};
    /// # #[tokio::main] async fn main() {
    /// # let client = QvaPayClient::new("key".to_string(), Environment::Sandbox);
    /// let req = CreateInvoiceRequest {
    ///     amount: 10.0,
    ///     description: "Test".to_string(),
    ///     remote_id: "123".to_string(),
    ///     webhook: None,
    ///     products: None,
    ///     expire_at: None,
    /// };
    /// let invoice = client.create_invoice(&req).await;
    /// # }
    /// ```
    pub async fn create_invoice(&self, request: &CreateInvoiceRequest) -> Result<InvoiceResponse, SdkError> {
        self.post("/v2/create_invoice", request).await
    }
}
