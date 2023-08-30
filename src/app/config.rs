use actix_web::web::ServiceConfig;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.app_data()
}
