use actix_web::{App, HttpServer};
use actix_files::Files;
mod pages;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static").show_files_listing())
            .service(pages::index::index)
            .service(pages::wallets::wallets)
            .service(pages::pgp::pgp)
            .service(pages::comm::comm)
            .service(pages::github::github)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
