pub mod ffi;

use crate::server::Server;
use actix_web::{get, put, web, HttpResponse, Responder, Result};
use trane::data::filter::ExerciseFilter;

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

#[put("/library/{library_id}/apply_filter")]
async fn apply_filter(
    data: web::Data<Server>,
    library_id: web::Path<String>,
    filter: web::Json<ExerciseFilter>,
) -> Result<HttpResponse> {
    data.apply_filter(library_id.as_str(), filter.into_inner())?;
    Ok(HttpResponse::Ok().body(""))
}

#[get("/library/{library_id}/get_exercise_batch")]
async fn get_exercise_batch(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    let batch = data.get_exercise_batch(library_id.as_str())?;
    Ok(HttpResponse::Ok().json(batch))
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
