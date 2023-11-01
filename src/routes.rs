use actix_web::{get, post, web, HttpResponse, Responder};
use crate::state::ServerState;

#[get("/")]
async fn hello(data: web::Data<ServerState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}