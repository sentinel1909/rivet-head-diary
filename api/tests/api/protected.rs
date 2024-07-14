// tests/api/protected.rs

// dependencies
use crate::helpers::spawn_app;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
}

#[tokio::test]
async fn protected_route_works() {
    // Arrange
    let app = spawn_app().await;
    let body = json!({
        "client_id": "test_id",
        "client_secret": "test_secret"
    });

    // Act: Login to get the JWT token
    let response_login = app.login(&body).await;

    // Parse the login response body to extract the token
    let response_login_body = response_login
        .text()
        .await
        .expect("Unable to retrieve the response text");
    let auth_response: AuthResponse =
        serde_json::from_str(&response_login_body).expect("Unable to parse JSON response");

    // Act: Call the protected route with the token
    let response_protected = app.protected(&auth_response.access_token).await;

    // Assert
    let status = response_protected.status().as_u16();
    assert_eq!(status, 200);

    let response_protected_body = response_protected
        .text()
        .await
        .expect("Unable to retrieve the response text");
    let expected_response = "Welcome to the protected area, test_id!";
    assert_eq!(response_protected_body, expected_response);
}
