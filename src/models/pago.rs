use serde::{Deserialize, Serialize};

/// Request to create a new invoice (payment).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateInvoiceRequest {
    /// Amount to be paid.
    pub amount: f64,
    /// Description of the payment.
    pub description: String,
    /// External identifier from your system.
    pub remote_id: String,
    /// Optional webhook URL for payment notifications.
    pub webhook: Option<String>,
    /// Optional list of products.
    pub products: Option<Vec<ProductInvoice>>,
    /// Optional expiration date in ISO 8601 format.
    pub expire_at: Option<String>,
}

/// Product details within an invoice.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInvoice {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

/// Response containing the created invoice details.
#[derive(Deserialize, Debug, Clone)]
pub struct InvoiceResponse {
    pub app_id: String,
    pub amount: f64,
    pub description: String,
    pub remote_id: String,
    pub transaction_uuid: String,
    pub expire_at: Option<String>,
    pub url: String,
}
