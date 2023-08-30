use actix_web::web::ServiceConfig;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};


use crate::handlers;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(handlers::image::random_image);
}

