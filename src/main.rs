use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use env_logger;

mod message;

pub struct AppState {
    pub webhook: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    env_logger::init();
    std::env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK enviorment variable not set");

    HttpServer::new(move || {
        let webhook = std::env::var("DISCORD_WEBHOOK").unwrap();
        App::new()
            .data(AppState {
                webhook: webhook,
            })
            .wrap(middleware::Logger::default())
            .route("/message", web::post().to(message::message))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
