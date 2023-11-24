use crate::server::Server;
use actix_web::{get, web, HttpResponse, Responder, Result};

#[get("/library/{library_id}/open")]
async fn open_library(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    data.open_library(library_id.as_str())?;
    Ok(HttpResponse::Ok().body(""))
}

#[get("/library/{library_id}/close")]
async fn close_library(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    data.close_library(library_id.as_str())?;
    Ok(HttpResponse::Ok().body(""))
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
