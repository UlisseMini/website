use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;

use crate::AppState;

use awc::Client;
use serde::Serialize;
use std::str;

const MAX_SIZE: usize = 1800; // max payload size is 2k (discord size limit - prefix)

#[derive(Serialize)]
struct Message<'a> {
    content: &'a str,
}

// TODO: Add rate limiting
pub async fn message(
    mut payload: web::Payload,
    req: HttpRequest,
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
        content: &(make_prefix(req) + str::from_utf8(&bytes)?)
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

fn get_user_agent<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("User-Agent")?.to_str().ok()
}

fn make_prefix(req: HttpRequest) -> String {
    let conn_info = req.connection_info();

    format!(
        "`{} - {}`\n",
        conn_info.remote().unwrap_or("???"),
        get_user_agent(&req).unwrap_or("???"),
    )
}


