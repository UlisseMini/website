use actix_files as fs;
use actix_web::{App, HttpServer, middleware};
use env_logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(middleware::Logger::default())
        .service(fs::Files::new("/", "./public").index_file("index.html")))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
