use actix_web::{http::header::ContentType, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::{GeneralName, HostCategory, NewHost},
    utils::{error_chain_fmt, ResponseData},
};

#[derive(serde::Deserialize)]
pub struct BodyData {
    name: String,
    category: String,
}

impl TryFrom<BodyData> for NewHost {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let name = GeneralName::parse(value.name)?;
        let category = HostCategory::parse(&value.category)?;

        Ok(NewHost { name, category })
    }
}

#[derive(thiserror::Error)]
pub enum PostHostError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PostHostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PostHostError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostHostError::ValidationError(_) => StatusCode::BAD_REQUEST,
            PostHostError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Add a new room"
    skip(pool, body),
)]
#[post("/hosts")]
pub async fn add_hosts(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostHostError> {
    let new_host: NewHost = body.0.try_into().map_err(PostHostError::ValidationError)?;
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from pool")?;
    let host_id = insert_host(&mut transaction, &new_host)
        .await
        .context("Failed to insert new hpst in the database.")?;
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store new host.")?;

    let data = ResponseData {
        data: host_id,
        code: StatusCode::OK.as_u16(),
        message: format!("Successfully created new host {}", new_host.name.as_ref()),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(data))
}

#[tracing::instrument(
    name = "Saving new host details in database.",
    skip(transaction, new_host)
)]
pub async fn insert_host(
    transaction: &mut Transaction<'_, Postgres>,
    new_host: &NewHost,
) -> Result<Uuid, sqlx::Error> {
    let host_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO hosts (id, name, category)
        VALUES ($1, $2, $3)
        "#,
        host_id,
        new_host.name.as_ref(),
        new_host.category.as_ref(),
    );
    transaction.execute(query).await?;

    Ok(host_id)
}
