use std::fmt::format;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::models::image;
use crate::types::AppStateData;

#[derive(Debug, MultipartForm)]
pub struct ImageFormData {
    //#[multipart(rename = "file")]
    pub title: Text<String>,
    pub imagefile: TempFile,
}

#[get("/image")]
pub async fn random_image(state: AppStateData) -> actix_web::Result<impl Responder> {
    let images = sqlx::query_as::<_, image::Image>("select * from image;")
        .fetch_all(&state.db)
        .await
        .unwrap();

    println!("{:?}", images);

    Ok(HttpResponse::Ok().json(images))
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {
    message: String,
}

#[post("/image")]
pub async fn upload_image(
    state: AppStateData,
    MultipartForm(form): MultipartForm<ImageFormData>,
) -> actix_web::Result<impl Responder> {
    //println!("{:?}", form);
    println!("image post received!");

    let image = sqlx::query!(
        "INSERT INTO image (name, url, width, height) VALUES ($1, $2, $3, $4) RETURNING name, id",
        form.title.into_inner(),
        "https://picsum.photos/200/300",
        200,
        300,
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    let res = Res {
        message: format!(
            "create image with the id: {} and the name {} for you :)",
            image.id, image.name
        ),
    };

    Ok(HttpResponse::Ok().json(res))
}
