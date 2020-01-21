use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;

use crate::AppState;

use awc::Client;
use serde::Serialize;
use std::str;

const MAX_SIZE: usize = 262_144; // max payload size is 16k

#[derive(Serialize)]
struct Message<'a> {
    content: &'a str,
}

// TODO: Add rate limiting
pub async fn message(
    mut payload: web::Payload,
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        // limit max size of in-memory payload
        if (bytes.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        bytes.extend_from_slice(&chunk);
    }

    let webhook = &state.get_ref().webhook;
    let json = Message {
        content: &str::from_utf8(&bytes)?,
    };
    let client = Client::default();

    let response = client
        .post(webhook) // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send_json(&json) // <- Send http request
        .await?;

    let status = response.status();

    if status.is_success() {
        return Ok(HttpResponse::Ok().body("Submitted"));
    } else {
        Err(Error::from(
            HttpResponse::InternalServerError().body("Webhook push failed"),
        ))
    }
}
