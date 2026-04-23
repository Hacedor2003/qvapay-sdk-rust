use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::auth::*;

impl QvaPayClient {
    /// Inicia sesión en QvaPay.
    ///
    /// # Ejemplo
    /// ```no_run
    /// # use qvapay_sdk::{QvaPayClient, Environment, models::auth::AuthLoginRequest};
    /// # #[tokio::main] async fn main() {
    /// # let client = QvaPayClient::new("key".to_string(), Environment::Sandbox);
    /// let req = AuthLoginRequest {
    ///     email: "user@example.com".to_string(),
    ///     password: "password123".to_string(),
    ///     remember: None,
    ///     two_factor_code: None,
    /// };
    /// let response = client.login(&req).await;
    /// # }
    /// ```
    pub async fn login(&self, request: &AuthLoginRequest) -> Result<AuthLoginResponse, SdkError> {
        self.post("/auth/login", request).await
    }

    /// Registra un nuevo usuario en QvaPay.
    pub async fn register(&self, request: &AuthRegisterRequest) -> Result<GeneralMessageResponse, SdkError> {
        self.post("/auth/register", request).await
    }

    /// Verifica el email del usuario confirmando el PIN enviado durante el registro.
    pub async fn register_confirmation(&self, request: &ConfirmRegistrationRequest) -> Result<AuthRegisterResponse, SdkError> {
        self.post("/auth/confirm-registration", request).await
    }

    /// Verifica si el token de sesión actual es válido. Útil para validar la autenticación desde middleware o clientes externos.
    pub async fn check_auth(&self) -> Result<CheckLoginResponse, SdkError> {
        self.post("/auth/check",&{}).await
    }

    /// Cierra la sesión actual del usuario eliminando el token de acceso.
    pub async fn logout(&self) -> Result<GeneralMessageResponse, SdkError> {
        self.get("/auth/logout").await
    }
}
