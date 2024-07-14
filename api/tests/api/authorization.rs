// tests/api/authorization.rs

// dependencies
use crate::helpers::spawn_app;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
}

#[tokio::test]
async fn authorization_works() {
    // Arrange
    let app = spawn_app().await;
    let body = json!({"client_id":"test_id", "client_secret":"test_secret"});

    // Act
    let response = app.login(&body).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let response_body = response
        .text()
        .await
        .expect("Unable to retrieve the response text.");
    let auth_response: AuthResponse =
        serde_json::from_str(&response_body).expect("Unable to parse JSON response");

    assert!(
        !auth_response.access_token.is_empty(),
        "access_token is empty"
    );
    assert_eq!(
        auth_response.token_type, "Bearer",
        "token_type is not 'Bearer'"
    );
}
