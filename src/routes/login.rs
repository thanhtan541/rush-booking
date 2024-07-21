use actix_web::{http::header::ContentType, post, web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    domain::CustomerEmail,
    utils::{error_chain_fmt, ResponseData},
};

#[derive(serde::Deserialize)]
struct BodyData {
    username: String,
    password: Secret<String>,
}

struct Credentials {
    username: CustomerEmail,
    password: Secret<String>,
}

impl TryFrom<BodyData> for Credentials {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let BodyData { username, password } = value;
        let username = CustomerEmail::parse(username)?;

        Ok(Credentials { username, password })
    }
}

#[tracing::instrument(
    name = "User login",
    skip(body, pool),
    fields(
        username=tracing::field::Empty,
        user_id=tracing::field::Empty,
    )
)]
#[post("/login")]
pub async fn login(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, LoginError> {
    let credentials: Credentials = body.0.try_into().map_err(LoginError::AuthError)?;

    let data = ResponseData {
        data: "token",
        code: StatusCode::OK.as_u16(),
        message: format!("Successfully login"),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(data))
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("{0}")]
    AuthError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::AuthError(_) => StatusCode::BAD_REQUEST,
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
