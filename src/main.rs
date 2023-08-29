use actix_cors::Cors;
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        text::Text,
        MultipartForm,
    },
    Multipart,
};
use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/image")]
async fn random_image(state: Data<AppState>) -> actix_web::Result<impl Responder> {
    let images = sqlx::query_as::<_, Image>("select * from images")
        .fetch_one(&state.db)
        .await
        .unwrap();
    println!("{:?}", images);

    Ok(HttpResponse::Ok().body("getting random image for you!!!"))
}

#[derive(Debug, MultipartForm)]
struct ImageFormData {
    //#[multipart(rename = "file")]
    title: Text<String>,
    imagefiles: TempFile,
}

#[post("/imagetest")]
async fn upload_image_test(
    MultipartForm(form): MultipartForm<ImageFormData>,
) -> actix_web::Result<impl Responder> {
    //println!("{:?}", form);
    println!("image post received!");
    Ok(HttpResponse::Ok().body("uploading image for you!!!"))
}

#[derive(Debug, MultipartForm)]
struct TestFormData {
    title: Text<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestFormResponse {
    title: String,
}

#[post("/formtest")]
async fn formtest(
    MultipartForm(form): MultipartForm<TestFormData>,
) -> actix_web::Result<impl Responder> {
    println!("image post received!");
    let res = TestFormResponse {
        title: form.title.into_inner(),
    };
    Ok(HttpResponse::Ok().json(res))
}

#[post("/image")]
async fn upload_image(
    state: Data<AppState>,
    //MultipartForm(form): MultipartForm<ImageFormData>,
) -> actix_web::Result<impl Responder> {
    //println!("{:?}", form);
    println!("image post received!");
    Ok(HttpResponse::Ok().body("uploading image for you!!!"))
}

#[derive(sqlx::FromRow, Debug)]
struct Image {
    id: i32,
    name: String,
    url: String,
    width: i32,
    height: i32,
}

struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = PgPool::connect("postgres://root:root@localhost:5432/imageservice")
        .await
        .expect("Failed to connect to Postgres.");

    let host = "127.0.0.1";
    let port = 5000;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!(
        "starting server at adress {}:{} or http://localhost:{port}",
        host, port
    );
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(db.clone())
            .service(hello)
            .service(echo)
            .service(random_image)
            .route("/hey", web::get().to(manual_hello))
            .service(upload_image)
            .service(upload_image_test)
            .service(formtest)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
