use crate::helpers::spawn_app;

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
async fn add_new_room_to_a_hotel_should_success() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name":"Single bed room",
        "description":"Single beds room with private pool",
        "number_of_beds": 1,
    });

    let response = app.post_rooms(&body).await;

    assert!(response.status().is_success());
}
