use uuid::Uuid;

use crate::helpers::{get_response_data_from_json, spawn_app};

#[tokio::test]
async fn add_room_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "named":"Double beds room",
                "beds":"<p>Newsletter body as HTML </p>"
            }),
            "invalid data format",
        ),
        (
            serde_json::json!({
                "name":"Double beds room",
                "number_of_beds": 2,
            }),
            "missing description",
        ),
        (
            serde_json::json!({
                "description":"Single beds room with private pool",
                "number_of_beds": 1,
            }),
            "missing name",
        ),
        (
            serde_json::json!({
                "name":"Double beds room",
                "description":"Single beds room with private pool",
                "number_of_beds": 1,
            }),
            "missing host id",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_rooms(&invalid_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn add_new_room_to_a_existed_hotel_should_fail() {
    let app = spawn_app().await;
    let not_existed_host_id = Uuid::new_v4();
    let body = serde_json::json!({
        "name":"Single bed room",
        "description":"Single beds room with private pool",
        "number_of_beds": 1,
        "host_id": not_existed_host_id,
    });

    let response = app.post_rooms(&body).await;
    assert!(response.status().is_server_error());
}

#[tokio::test]
async fn add_new_room_to_a_hotel_should_success() {
    let app = spawn_app().await;
    // Create a new host
    let body_for_host = serde_json::json!({
        "name":"Single bed room",
        "category":"hotel",
    });
    let response = app.post_hosts(&body_for_host).await;
    assert!(response.status().is_success());
    let host_json = get_response_data_from_json(response).await;
    let body = serde_json::json!({
        "name":"Single bed room",
        "description":"Single beds room with private pool",
        "number_of_beds": 1,
        "host_id": host_json.data,
    });

    let response = app.post_rooms(&body).await;
    assert!(response.status().is_success());
    let room_json = get_response_data_from_json(response).await;
    assert!(room_json
        .message
        .as_str()
        .contains("Successfully created new room"));
}
