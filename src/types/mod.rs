
use actix_web::web::Data;
use sqlx::{Pool, Postgres};


pub type AppStateData= Data<AppState>;
pub struct AppState {
    pub db: Pool<Postgres>,
}

