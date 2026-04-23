use serde::{Deserialize, Serialize};

pub mod gift_card;
pub mod phone_package;

pub use phone_package::*;
pub use gift_card::*;

/// Producto de la tienda.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreProduct {}

/// Respuesta de la Tienda en general
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreResponse {
    message: String,
    data: Vec<StoreProduct>,
}

