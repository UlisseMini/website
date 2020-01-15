use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use listenfd::ListenFd;

mod message;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/message", web::post().to(message::message))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    });

        server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
