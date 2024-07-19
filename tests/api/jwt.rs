use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rush_booking::authentication::{sign, Options, Payload, DEFAULT_TOKEN_TTL};

#[tokio::test]
async fn should_produce_different_signature_for_different_payloads() {
    let secret = "shhh";
    let options = Options::new();
    let jwt_one = sign("abc", secret, Some(&options));
    let jwt_two = sign("def", secret, Some(&options));

    let sign_one = jwt_one.split(".").collect::<Vec<&str>>()[2];
    let sign_two = jwt_two.split(".").collect::<Vec<&str>>()[2];

    assert_ne!(sign_one, sign_two);
}

#[tokio::test]
async fn should_add_expiry_to_the_payload() {
    let secret = "shhh";
    let options = Options::new();
    let jwt_one = sign("abc", secret, Some(&options));

    let payload_one = jwt_one.split(".").collect::<Vec<&str>>()[1];
    // Decode the payload and get "exp" key and its value is integer
    let decoder = URL_SAFE_NO_PAD
        .decode(payload_one)
        .expect("Failed to decode payload string");
    let payload_str = String::from_utf8(decoder).expect("Invalid UTF8 format");
    let payload: Payload =
        serde_json::from_str(&payload_str).expect("Failed to parse to Payload struct");

    assert_eq!(payload.exp, DEFAULT_TOKEN_TTL);
}
