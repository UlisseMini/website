use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;

use std::fs::OpenOptions;
use std::io::prelude::*;

const MAX_SIZE: usize = 262_144; // max payload size is 16k

fn get_user_agent<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("User-Agent")?.to_str().ok()
}

fn make_prefix(req: HttpRequest) -> String {
    use chrono::Utc;
    use chrono_tz::America::New_York;

    let date = Utc::now();

    let conn_info = req.connection_info();

    // TODO: Fix this, it is reporting incorrect time!
    format!(
        "addr: {}\nuser-agent: {}\ndate: {}\n",
        conn_info.remote().unwrap_or("???"),
        get_user_agent(&req).unwrap_or("???"),
        date.with_timezone(&New_York)
            .format("%D %I:%M:%S %p")
            .to_string()
    )
}

// TODO: Add rate limiting
// TODO: Make server errors nicer (currently frontend says 500 error not error message)
// TODO: Record Date of messages
// TODO: Save messages in a more efficent way
pub async fn message(mut payload: web::Payload, req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        // limit max size of in-memory payload
        if (bytes.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        bytes.extend_from_slice(&chunk);
    }

    // TODO: Make async
    // TODO: Compress file when it reaches a specific size?
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("messages.txt")?;

    bytes.extend_from_slice(b"\n--------------------------------------\n");
    file.write_all(&make_prefix(req).as_bytes())?;
    file.write_all(&bytes)?;

    Ok(HttpResponse::Ok().body("Submitted"))
}

