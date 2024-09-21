use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::{PgConnection, PgPool};
use crate::{routes::health_check, routes::subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture 'connection' from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            // Middlewares are added using the `wrap` method on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}