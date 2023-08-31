use actix_web::web::ServiceConfig;


use crate::handlers;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(handlers::image::random_image);
}

