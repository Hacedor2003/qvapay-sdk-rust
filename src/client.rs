use reqwest::{Client as HttpClient, Method};
use serde::{de::DeserializeOwned, Serialize};
use crate::error::{SdkError, ApiErrorResponse};

#[derive(Debug, Clone)]
pub enum Environment {
    Sandbox,
    Production,
    Custom(String),
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Sandbox => "https://sandbox.api.qvapay.com",
            Environment::Production => "https://api.qvapay.com",
            Environment::Custom(url) => url,
        }
    }
}

pub struct QvaPayClient {
    api_key: String,
    base_url: String,
    http_client: HttpClient,
}

impl QvaPayClient {
    pub fn new(api_key: String, environment: Environment) -> Self {
        Self {
            api_key,
            base_url: environment.base_url().to_string(),
            http_client: HttpClient::new(),
        }
    }

    pub(crate) async fn request<T, R>(&self, method: Method, path: &str, body: Option<&T>) -> Result<R, SdkError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut rb = self.http_client.request(method, &url)
            .header("X-API-Key", &self.api_key);

        if let Some(b) = body {
            rb = rb.json(b);
        }

        let response = rb.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json::<R>().await?)
        } else {
            let error_body = response.text().await.unwrap_or_default();
            let message = if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&error_body) {
                api_err.message.or(api_err.error).unwrap_or_else(|| "Unknown API error".to_string())
            } else {
                error_body
            };

            Err(SdkError::Api {
                status: status.as_u16(),
                message,
            })
        }
    }

    pub(crate) async fn get<R>(&self, path: &str) -> Result<R, SdkError>
    where
        R: DeserializeOwned,
    {
        self.request::<(), R>(Method::GET, path, None).await
    }

    pub(crate) async fn post<T, R>(&self, path: &str, body: &T) -> Result<R, SdkError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        self.request(Method::POST, path, Some(body)).await
    }
}
