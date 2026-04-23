use serde::{Deserialize, Serialize};

/// Paquete de recarga telefonica de Cuba.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhonePackageProduct {
    pub id: i32,
    pub name: String,
    pub logo: String,
    pub details: String,
    pub price: i32,
    pub gold_price: i32,
    pub external: bool,
    pub external_amount: i32,
    pub period: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhonePackageResponse {
    pub phone_packages: Vec<PhonePackageProduct>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuyPhonePackageRequest {
    pub phone_package_id: i32,
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuyPhonePackageResponse {
    pub message: String,
    pub transaction_uuid: String,
    pub buyedService: String,
}
