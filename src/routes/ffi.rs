use crate::server::Server;
use actix_web::{get, put, web, HttpResponse, Result};
use trane::data::ffi::{filter::ExerciseFilter, ExerciseManifest};

#[put("/library/{library_id}/ffi/apply_filter")]
async fn apply_filter(
    data: web::Data<Server>,
    library_id: web::Path<String>,
    filter: web::Json<ExerciseFilter>,
) -> Result<HttpResponse> {
    data.apply_filter(library_id.as_str(), filter.into_inner().into())?;
    Ok(HttpResponse::Ok().body(""))
}

#[get("/library/{library_id}/ffi/get_exercise_batch")]
async fn get_exercise_batch(
    data: web::Data<Server>,
    library_id: web::Path<String>,
) -> Result<HttpResponse> {
    let batch: Vec<ExerciseManifest> = data
        .get_exercise_batch(library_id.as_str())?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(HttpResponse::Ok().json(batch))
}
