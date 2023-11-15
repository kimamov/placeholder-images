use actix_web::{get, HttpResponse, Responder};
use ramhorns::{Template, Content};

use crate::types::AppStateData;

#[derive(Content)]
struct Home {
    //posts: Vec<String>
    title: String,
}

#[get("/")]
pub async fn index(state: AppStateData) -> actix_web::Result<impl Responder> {

    let source="<h1>{{title}}</h1><h2>welcome to my image service</h2>";

    let tpl=Template::new(source).unwrap();

    let rendered=tpl.render(&Home {title: "Test passed".to_string()});


    Ok(HttpResponse::Ok().body(rendered))
}

