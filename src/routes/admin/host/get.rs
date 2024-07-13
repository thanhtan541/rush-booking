use actix_web::{get, http::header::ContentType, web, HttpResponse};

use crate::utils::ResponseData;

#[derive(serde::Deserialize)]
pub struct Info {
    host_id: u8,
}

#[tracing::instrument(
    name = "Retrieve host information"
    skip(info),
)]
#[get("/hosts/{host_id}")]
pub async fn get_hosts(info: web::Path<Info>) -> Result<HttpResponse, actix_web::Error> {
    let Info { host_id } = info.into_inner();

    let response = ResponseData {
        data: format!("Accessing host id : {}", host_id),
        code: 200,
        message: format!("Successfully retrieving data"),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response))
}
