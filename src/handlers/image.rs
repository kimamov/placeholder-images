use std::fmt::Debug;
use std::path::PathBuf;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::models::image::{self};
use crate::services::image::create_thumbnail;
use crate::types::AppStateData;

#[derive(Debug, MultipartForm)]
pub struct ImageFormData {
    //#[multipart(rename = "file")]
    pub title: Text<String>,
    pub file: TempFile,
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

#[post("/post")]
pub async fn upload_image(
    state: AppStateData,
    MultipartForm(form): MultipartForm<ImageFormData>,
) -> impl Responder {
    //println!("{:?}", form);
    println!("image post received!");
    const MAX_FILE_SIZE: usize = 1024 * 1024 * 10;

    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest().body(format!("File is too large."))
        }
        _ => {}
    }

    let file_name: &str = form
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let mut file_path = PathBuf::from("static/");
    file_path.push(&file_name);

    println!("{}", file_path.display());

    match form.file.file.persist(&file_path) {
        Ok(_) => match create_thumbnail(file_path.as_os_str().to_str().unwrap()) {
            Ok(path) => {
                println!("create thumbnail at path {}", path);
                match sqlx::query!(
                                    "INSERT INTO image (name, url, width, height, thumbnail) VALUES ($1, $2, $3, $4, $5) RETURNING name, id, url, thumbnail",
                                    form.title.into_inner(),
                                    format!("/static/{}",file_name),
                                    200,
                                    300,
                                    format!("/static/{}_thumbnail", file_name)
                                )
                .fetch_one(&state.db)
                .await
                {
                    Ok(rec) => HttpResponse::Ok().body(format!(
                        "
                        <p>succesfully created post: {}</p>
                        <img src='{}'/>
                        ",
                        rec.name,
                        rec.url
                    )),
                    Err(e) => {
                        println!("{}", e);
                        HttpResponse::InternalServerError().body("db error")
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                HttpResponse::InternalServerError().body("failed to create thumbnail")
            }
        },
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("file error")
        }
    }
}
