use actix_web::{get, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    moji: String,
    num: isize,
    arr: Vec::<isize>,
}

#[get("/getjson")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        moji: "test".to_string(),
        num: 100,
        arr: vec![1, 2, 3],
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
