//! # QvaPay SDK
//!
//! Un SDK en Rust para la API de Pagos QVAPAY.
//!
//! ## Ejemplo de uso
//!
//! ```rust
//! use qvapay_sdk::{QvaPayClient, Environment, models::CreateInvoiceRequest};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = QvaPayClient::new("tu_api_key".to_string(), Environment::Sandbox);
//!
//!     let invoice_req = CreateInvoiceRequest {
//!         amount: 10.0,
//!         description: "Pago de prueba".to_string(),
//!         remote_id: "orden_123".to_string(),
//!         webhook: None,
//!         products: None,
//!         expire_at: None,
//!     };
//!
//!     match client.create_invoice(&invoice_req).await {
//!         Ok(invoice) => println!("URL de pago: {}", invoice.url),
//!         Err(e) => eprintln!("Error: {:?}", e),
//!     }
//! }
//! ```

pub mod client;
pub mod error;
pub mod api;
pub mod models;

pub use client::{QvaPayClient, Environment};
pub use error::SdkError;
