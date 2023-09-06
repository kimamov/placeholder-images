use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::models::image;
use crate::types::AppStateData;

#[derive(Debug, MultipartForm)]
struct ImageFormData {
    //#[multipart(rename = "file")]
    pub title: Text<String>,
    pub imagefile: TempFile,
}

#[get("/image")]
pub async fn random_image(state: AppStateData) -> actix_web::Result<impl Responder> {
    /* let images = sqlx::query_as::<_, image::Image>("select * from image;")
       .fetch_all(&state.db)
       .await
       .unwrap();
    */

    let images = sqlx::query_as!(image::Image, "select * from image;")
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
        r#"INSERT INTO image ("name", "url", "width", "height") VALUES ( ?, ?, ?, ? )"#,
        "form.title",
        "https://www.google.com",
        100,
        100,
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    let res = Res {
        message: "uploading image for you!!!".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}
