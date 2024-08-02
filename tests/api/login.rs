use rush_booking::authentication::JwtResponse;
use uuid::Uuid;

use crate::helpers::{get_response_data_from_json, spawn_app};

#[tokio::test]
async fn login_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "password":"sdfasfsfsdf"
            }),
            "missing username",
        ),
        (
            serde_json::json!({
                "username":"sfsdfsfsdf",
            }),
            "missing password",
        ),
        (
            serde_json::json!({
                "username":"sfasfdssss",
                "password": "asdasd",
            }),
            "invalid username",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_login(&invalid_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn reponse_has_no_token_after_login_with_wrong_credential() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "username": &app.test_user.username,
        //Fake mismatch password
        "password": Uuid::new_v4(),
    });

    let response = app.post_login(&body).await;

    assert!(response.status().is_client_error());
    let room_json = get_response_data_from_json::<String>(response).await;
    assert!(room_json.message.as_str().contains("Invalid credentials"));
}

#[tokio::test]
async fn reponse_payload_has_token_after_login_success() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&body).await;

    assert!(response.status().is_success());
    let login_resp: JwtResponse = response.json().await.unwrap();
    assert!(login_resp.access_token.as_str().contains("access_token"));
}
