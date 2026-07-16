use crate::doorbell::{DOORBELL_IP, DOORBELL_USERNAME};
use dioxus::{
    fullstack::{
        Query,
        body::Body,
        http::header,
        response::{self, IntoResponse},
    },
    logger::tracing,
    prelude::*,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DoorbellPictureParams {
    password: String,
    id: u32,
}

pub async fn get_doorbell_picture(
    Query(DoorbellPictureParams { password, id }): Query<DoorbellPictureParams>,
) -> response::Response {
    let url = format!(
        "http://{DOORBELL_IP}/cgi-bin/images_cgi?channel=0&user={DOORBELL_USERNAME}&pwd={password}&{id}",
    );
    let reqwest_res = match dioxus_fullstack::reqwest::get(&url).await {
        Ok(res) => res,
        Err(err) => {
            tracing::warn!("Failed to get doorbell picture: {}", err);
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let filename = format!("doorbell_picture_{}.jpg", id);
    let builder = dioxus_fullstack::response::Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            reqwest_res
                .headers()
                .get(header::CONTENT_TYPE)
                .unwrap_or(&header::HeaderValue::from_static("image/jpeg")),
        )
        .header(
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{filename}\""),
        );

    let bytes = match reqwest_res.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => {
            tracing::warn!("Failed to get doorbell picture: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    builder.body(Body::from(bytes)).unwrap().into_response()
}
