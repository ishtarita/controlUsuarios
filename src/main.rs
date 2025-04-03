use actix_web::{App, HttpServer};
use routes::user_routes::configure as configure_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(configure_users) // Agrega rutas de usuarios
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
