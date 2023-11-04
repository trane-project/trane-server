use crate::server::Server;
use actix_web::{get, post, web, HttpResponse, Responder, Result};

#[get("/open/{library_id}")]
async fn open_library(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    data.open_library(library_id.as_str())?;
    Ok(HttpResponse::Ok().body(""))
}

#[get("/close/{library_id}")]
async fn close_library(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    data.close_library(library_id.as_str())?;
    Ok(HttpResponse::Ok().body(""))
}

#[post("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
