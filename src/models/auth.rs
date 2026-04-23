use serde::{Deserialize, Serialize};

pub struct MerchantSecret {
    pub app_id: String,
    pub app_secret: String,
}

/// Request for login operation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
    pub two_factor_code: Option<String>,
}

/// Response for login when 2FA is required.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse2FA {
    pub info: String,
    pub notified: bool,
    pub has_otp: bool,
}

/// Successful login response.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub me: Profile,
}

/// User profile information.
#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub username: String,
    pub balance: String,
}

/// Request for registration operation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthRegisterRequest {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub invite: Option<String>,
    pub terms: String,
}

/// Successful registration response.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthRegisterResponse {
    pub message: String,
    pub user: Profile,
}

/// Successful registration confirm response.
#[derive(Deserialize, Debug, Clone)]
pub struct GeneralMessageResponse {
    pub message: String,
}

/// Successful check login.
#[derive(Deserialize, Debug, Clone)]
pub struct CheckLoginResponse {
    pub success: String,
}

/// Confirm Registration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmRegistrationRequest {
    pub uuid: String,
    pub email: String,
    pub pin: String,
}
