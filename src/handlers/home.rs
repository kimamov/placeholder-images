use actix_web::{get, HttpResponse, Responder};

use crate::types::AppStateData;

#[get("/")]
pub async fn index(state: AppStateData) -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok().body("welcome to the image api"))
}
