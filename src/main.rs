use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;

#[get("/subtract/{a}/{b}")]
async fn subtract(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = a - b;
    HttpResponse::Ok().body(result.to_string())
}
#[get("/add/{a}/{b}")]
async fn add(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = a + b;
    HttpResponse::Ok().body(result.to_string())
}
#[get("/multiply/{a}/{b}")]
async fn multiply(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = a * b;
    HttpResponse::Ok().body(result.to_string())
}
#[get("/divide/{a}/{b}")]
async fn divide(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = a / b;
    HttpResponse::Ok().body(result.to_string())
}
#[get("/pow/{a}/{b}")]
async fn pow(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let b = b as u32;
    let result = a.pow(b);
    HttpResponse::Ok().body(result.to_string())
}
#[get("/nrt/{a}/{b}")]
async fn nrt(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let b = b as f32;
    let a = a as f32;
    let result = a.powf(1.0/b);
    HttpResponse::Ok().body(result.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)    
            .service(pow) 
            .service(nrt)
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

