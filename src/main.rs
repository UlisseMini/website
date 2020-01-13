use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Error, error
};
use env_logger;
use futures::StreamExt;

use std::fs::OpenOptions;
use std::io::prelude::*;

const MAX_SIZE: usize = 262_144; // max payload size is 16k

fn get_user_agent<'a>(req: &'a HttpRequest) -> Option<&'a str> {
        req.headers().get("User-Agent")?.to_str().ok()
}

// TODO: Add rate limiting
// TODO: Make server errors nicer (currently frontend says 500 error not error message)
// TODO: Record Date of messages
// TODO: Save messages in a more efficent way
async fn message(mut payload: web::Payload, req: HttpRequest) -> Result<HttpResponse, Error> {
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


    let conn_info = req.connection_info();
    let suffix = format!("\n\n{} | {}\n\n",
        conn_info.remote().unwrap_or("???"),
        get_user_agent(&req).unwrap_or("???"));
    bytes.extend_from_slice(suffix.as_bytes());
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
