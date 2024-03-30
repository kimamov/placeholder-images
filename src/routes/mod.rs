use actix_web::web::ServiceConfig;

use crate::handlers;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(handlers::home::index)
        .service(handlers::home::post_detail)
        .service(handlers::image::random_image)
        .service(handlers::image::upload_image);
}
