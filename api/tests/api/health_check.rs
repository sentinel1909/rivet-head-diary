// tests/api/health_check.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let response = app
        .api_client
        .get(format!("{}/api/health_check", app.address))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    assert_eq!(status, 200);
}
