mod handlers;
mod models;
mod errors;
mod db;
mod auth;

use actix_web::{web, App, HttpServer, middleware};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Starting portfolio API server...");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                    )
                    .service(
                        web::scope("/projects")
                            .route("", web::get().to(handlers::projects::list_projects))
                            .route("/{id}", web::get().to(handlers::projects::get_project))
                            .route("", web::post().to(handlers::projects::create_project))
                            .route("/{id}", web::put().to(handlers::projects::update_project))
                            .route("/{id}", web::delete().to(handlers::projects::delete_project))
                    )
                    .service(
                        web::scope("/skills")
                            .route("", web::get().to(handlers::skills::list_skills))
                            .route("/{id}", web::get().to(handlers::skills::get_skill))
                            .route("", web::post().to(handlers::skills::create_skill))
                            .route("/{id}", web::put().to(handlers::skills::update_skill))
                            .route("/{id}", web::delete().to(handlers::skills::delete_skill))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
