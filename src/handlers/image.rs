
use actix_multipart::form::{
        tempfile::TempFile,
        text::Text,
        MultipartForm,
    };

use actix_web::{
    get,
    post,
    web,
    HttpResponse, Responder,
};

use crate::types::AppStateData;
use crate::models::image;

#[derive(Debug, MultipartForm)]
struct ImageFormData {
    //#[multipart(rename = "file")]
    title: Text<String>,
    imagefile: TempFile,
}




#[get("/image")]
pub async fn random_image(state: AppStateData) -> actix_web::Result<impl Responder> {
    let images = sqlx::query_as::<_, image::Image>("select * from images")
        .fetch_one(&state.db)
        .await
        .unwrap();
    println!("{:?}", images);

    Ok(HttpResponse::Ok().body("getting random image for you!!!"))
}


#[post("/image")]
pub async fn upload_image(
    state: AppStateData,
    MultipartForm(form): MultipartForm<ImageFormData>,
) -> actix_web::Result<impl Responder> {
    //println!("{:?}", form);
    println!("image post received!");
    Ok(HttpResponse::Ok().body("uploading image for you!!!"))
}

