use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCard {
    pub id: i32,
    pub uuid: String,
    pub slug: String,
    pub name: String,
    pub lead: String,
    pub color: String,
    pub tax: i32,
    pub tax_gold: i32,
    pub logo: String,
    pub sublogo: String,
    pub desc: String,
    pub meta: String,
    pub featured: bool,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Respuesta de la Tienda en las tarjetas de regalo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreGiftCardResponse {
    message: String,
    data: Vec<GiftCard>,
}

/// BodyParams for buy a gift card
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreGiftCardBuyRequest {
    code: String,
    amout: i32,
}

/// Response of a buy of a giftCard
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreGiftCardBuyResponse {
    message: String,
    data: StoreGiftCardBuyData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreGiftCardBuyData {
    transaction_uuid: String,
    buyedService_id: String,
    amout: f32,
    status: String,
}
