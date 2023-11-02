use crate::state::ServerState;
use actix_web::{get, post, web, HttpResponse, Responder};
use trane::Trane;

/// Opens the library with the given ID.
#[get("/open/{library_id}")]
async fn open_library(data: web::Data<ServerState>, library_id: web::Path<String>) -> HttpResponse {
    // Check if the library is already opened.
    let library_id = library_id.into_inner();
    if data.opened_libraries.lock().contains_key(&library_id) {
        return HttpResponse::Ok().body("");
    }

    // Check if the library exists in the library directory.
    let library_path = data.config.library_directory.join(library_id.clone());
    if !library_path.exists() {
        return HttpResponse::NotFound().body("");
    }

    // Open the library and add it to the map.
    let library = Trane::new(&library_path, &library_path);
    if let Err(e) = library {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    data.opened_libraries
        .lock()
        .insert(library_id, library.unwrap());
    HttpResponse::Ok().body("")
}

#[post("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
