use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::{
    BuyPhonePackageRequest, BuyPhonePackageResponse, PhonePackageResponse, StoreGiftCardBuyRequest,
    StoreGiftCardBuyResponse, StoreGiftCardResponse, StoreResponse,
};

impl QvaPayClient {
    /// Obtener la lista de productos disponibles en la tienda.
    pub async fn get_store_products(&self) -> Result<StoreResponse, SdkError> {
        self.get("/store").await
    }

    /// Obtener la lista de tarjetas de regalos disponibles en la tienda.
    pub async fn get_store_gift_card(&self) -> Result<StoreGiftCardResponse, SdkError> {
        self.get("/store/gift-card").await
    }

    /// Comprar una tarjeta de regalo en la tienda.
    pub async fn buy_store_gift_card(
        &self,
        data: StoreGiftCardBuyRequest,
        uuid: &str,
    ) -> Result<StoreGiftCardBuyResponse, SdkError> {
        self.post(&format!("/store/gift-card/{}", uuid), &data)
            .await
    }

    /// Obtener la lista de recargas de celulares para Cuba de la tienda
    pub async fn get_store_phone_package(&self) -> Result<PhonePackageResponse, SdkError> {
        self.get("store/phone_package").await
    }

    /// Comprar una recarga de telefono en la tienda.
    pub async fn buy_phone_package(
        &self,
        data: BuyPhonePackageRequest,
    ) -> Result<BuyPhonePackageResponse, SdkError> {
        self.post("/store/phone_package", &data).await
    }
}
