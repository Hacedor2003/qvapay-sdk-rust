use qvapay_sdk::{QvaPayClient, Environment, models::CreateInvoiceRequest};
use wiremock::matchers::{method, path, header};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

#[tokio::test]
async fn test_create_invoice_success() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "app_id": "test_app",
        "amount": 10.0,
        "description": "Test",
        "remote_id": "123",
        "transaction_uuid": "uuid",
        "expire_at": null,
        "url": "https://qvapay.com/pay/uuid"
    });

    Mock::given(method("POST"))
        .and(path("/v2/create_invoice"))
        .and(header("X-API-Key", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = QvaPayClient::new(Environment::Custom(mock_server.uri()));

    let req = CreateInvoiceRequest {
        amount: 10.0,
        description: "Test".to_string(),
        remote_id: "123".to_string(),
        webhook: None,
        products: None,
        expire_at: None,
    };

    let result = client.create_invoice(&req).await.unwrap();
    assert_eq!(result.url, "https://qvapay.com/pay/uuid");
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "message": "Invalid API Key"
    });

    Mock::given(method("GET"))
        .and(path("/v2/balance"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body))
        .mount(&mock_server)
        .await;

    let client = QvaPayClient::new( Environment::Custom(mock_server.uri()));

    let result = client.get_balance().await;

    match result {
        Err(qvapay_sdk::SdkError::Api { status, message }) => {
            assert_eq!(status, 401);
            assert_eq!(message, "Invalid API Key");
        },
        _ => panic!("Expected Api error"),
    }
}
