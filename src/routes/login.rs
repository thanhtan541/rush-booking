use actix_web::{
    error::InternalError, http::header::ContentType, post, web, HttpResponse, ResponseError,
};
use anyhow::anyhow;
use reqwest::StatusCode;
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    domain::CustomerEmail,
    utils::{error_chain_fmt, ResponseData},
};

#[derive(serde::Deserialize)]
struct BodyData {
    username: String,
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
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials: Credentials = body.0.try_into().map_err(|_| {
        InternalError::new(
            LoginError::AuthError(anyhow!("Invalid credential")),
            StatusCode::BAD_REQUEST,
        )
    })?;

    tracing::Span::current().record(
        "username",
        &tracing::field::display(&credentials.username.as_ref()),
    );
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            let data = ResponseData {
                data: "token",
                code: StatusCode::OK.as_u16(),
                message: format!("Successfully fetch token"),
            };

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(data))
        }
        Err(e) => {
            let e = match e {
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
            };
            let data = ResponseData {
                data: "",
                code: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Invalid credentials"),
            };

            let response = HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .json(data);
            Err(InternalError::from_response(e, response))
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
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
