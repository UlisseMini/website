use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Error, error
};
use env_logger;
use futures::StreamExt;

use std::fs::OpenOptions;
use std::io::prelude::*;

const MAX_SIZE: usize = 262_144; // max payload size is 16k

// TODO: Add rate limiting
async fn message(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("messages.txt")?;

    bytes.extend_from_slice(b"\n-------------------------------------------------------\n");
    file.write_all(&bytes)?;

    Ok(HttpResponse::Ok().body("Submitted"))

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/message", web::post().to(message))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
