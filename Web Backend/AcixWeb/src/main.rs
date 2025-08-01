
// use utoipa::openapi::response;

mod routes;  // Import the route modules

use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use routes::{auth_routes, get_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8082;
    println!("Listening on port {}", port);

    HttpServer::new(|| {
        App::new()
            .configure(get_routes::config)    // Register the get routes
            .configure(auth_routes::config)   // Register the auth routes
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
