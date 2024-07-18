use actix_web::{http::header::ContentType, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::{GeneralName, NewRoom},
    utils::{error_chain_fmt, ResponseData},
};

#[derive(serde::Deserialize)]
pub struct BodyData {
    name: String,
    host_id: Uuid,
    description: String,
    number_of_beds: u16,
}

impl TryFrom<BodyData> for NewRoom {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let BodyData {
            name,
            host_id,
            description,
            number_of_beds,
        } = value;
        let name = GeneralName::parse(name)?;

        Ok(NewRoom {
            name,
            host_id,
            description,
            number_of_beds,
        })
    }
}

#[derive(thiserror::Error)]
pub enum PostRoomError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PostRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PostRoomError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostRoomError::ValidationError(_) => StatusCode::BAD_REQUEST,
            PostRoomError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Add a new room"
    skip(pool, body),
)]
#[post("/rooms")]
pub async fn add_rooms(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostRoomError> {
    let new_room: NewRoom = body.0.try_into().map_err(PostRoomError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from pool")?;
    let room_id = insert_room(&mut transaction, &new_room)
        .await
        .context("Failed to insert new room in the database.")?;
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store new room.")?;

    let data = ResponseData {
        data: room_id,
        code: StatusCode::OK.as_u16(),
        message: format!("Successfully created new room {}", new_room.name.as_ref()),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(data))
}

#[tracing::instrument(
    name = "Saving new room details in database.",
    skip(transaction, new_room)
)]
pub async fn insert_room(
    transaction: &mut Transaction<'_, Postgres>,
    new_room: &NewRoom,
) -> Result<Uuid, sqlx::Error> {
    let room_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO rooms (id, host_id, name, description, number_of_beds)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        room_id,
        new_room.host_id,
        new_room.name.as_ref(),
        new_room.description,
        new_room.number_of_beds as i16,
    );
    transaction.execute(query).await?;

    Ok(room_id)
}
