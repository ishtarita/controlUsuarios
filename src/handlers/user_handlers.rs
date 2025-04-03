use actix_web::{get, Responder, HttpResponse};

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Lista de usuarios")
}
