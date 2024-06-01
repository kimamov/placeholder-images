use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use env_logger::Env;
use sqlx::postgres::PgPool;

mod app;
mod handlers;
mod models;
mod routes;
mod services;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = PgPool::connect("postgres://root:root@localhost:5432/imageservice")
        .await
        .expect("Failed to connect to Postgres.");

    let host = "127.0.0.1";
    let port = 5000;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!(
        "starting server at adress {}:{} or http://localhost:{port}",
        host, port
    );
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .service(fs::Files::new("/static", "static/").show_files_listing())
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(types::AppState { db: db.clone() }))
            .configure(app::config::init)
            .configure(routes::init)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
