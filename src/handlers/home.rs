use actix_web::{
    http::{header::ContentType, Error},
    route, web, HttpResponse, Responder, ResponseError,
};
use askama_actix::Template;

use crate::{
    models::image::{self, Image},
    types::AppStateData,
};

#[derive(Template)] // this will generate the code...
#[template(path = "home.html")] // using the template in this path, relative
struct Home {
    posts: Vec<Image>,
    title: String,
}

#[route("/", method = "GET", method = "HEAD")]
pub async fn index(state: AppStateData) -> actix_web::Result<impl Responder> {
    let images = sqlx::query_as::<_, image::Image>("select * from image;")
        .fetch_all(&state.db)
        .await
        .unwrap();

    println!("{:?}", images);

    let template = Home {
        title: "home".to_string(),
        posts: images,
    };

    match template.render() {
        Ok(template_string) => Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(template_string)),
        Err(_) => Ok(HttpResponse::InternalServerError()
            .reason("failed to render template to string")
            .finish()),
    }
}

#[derive(Template)]
#[template(path = "detail.html")]
struct DetailTemplate {
    pub post: Image,
}

#[route("/{id}", method = "GET", method = "HEAD")]
pub async fn post_detail(
    state: AppStateData,
    id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let image = sqlx::query_as::<_, image::Image>("SELECT * FROM image WHERE id = $1")
        .bind(id.clone())
        .fetch_one(&state.db)
        .await;

    match image {
        Ok(image_data) => {
            let template = DetailTemplate { post: image_data };

            match template.render() {
                Ok(template_string) => Ok(HttpResponse::Ok().body(template_string)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(e) => Ok(HttpResponse::NotFound().body(e.to_string())),
    }
}

#[derive(Template)]
#[template(path = "createPost.html")]
struct CreatePostTemplate {
    pub title: String,
}

#[route("/create", method = "GET", method = "HEAD")]
pub async fn post_create() -> actix_web::Result<impl Responder> {
    let template = CreatePostTemplate {
        title: "test".to_string(),
    };
    match template.render() {
        Ok(template_string) => Ok(HttpResponse::Ok().body(template_string)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
