use crate::helpers::spawn_app;

#[tokio::test]
async fn add_host_returns_400_for_invalid_data() {
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
            }),
            "missing category",
        ),
        (
            serde_json::json!({
                "category": "hotel",
            }),
            "missing name",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_hosts(&invalid_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn add_new_host_type_hotel_should_success() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name":"Single bed room",
        "category":"hotel",
    });

    let response = app.post_hosts(&body).await;
    let status_code = response.status().as_u16();

    assert!(response.status().is_success());

    let response_body = response.text().await.unwrap();
    assert!(response_body.contains(&status_code.to_string()));
}
