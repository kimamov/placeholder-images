use actix_web::{http::header::ContentType, route, HttpResponse, Responder};
use ramhorns::{Content, Ramhorns, Template};

use crate::{
    models::image::{self, Image},
    types::AppStateData,
};

#[derive(Content)]
struct Home {
    posts: Vec<Image>,
    title: String,
}

#[derive(Content)]
struct Page<T> {
    body: T,
}

#[route("/", method = "GET", method = "HEAD")]
pub async fn index(state: AppStateData) -> actix_web::Result<impl Responder> {
    let mut tpls: Ramhorns = Ramhorns::lazy("templates").unwrap();
    let tpl = tpls.from_file("home.mustache").unwrap();

    let images = sqlx::query_as::<_, image::Image>("select * from image;")
        .fetch_all(&state.db)
        .await
        .unwrap();

    println!("{:?}", images);

    // let rendered_page = tpl.render(&Page {
    //     body: Home {
    //         title: "Feed".to_string(),
    //         posts: images,
    //     },
    // });

    let rendered = tpl.render(&Home {
        title: "Test passed".to_string(),
        posts: images,
    });

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
