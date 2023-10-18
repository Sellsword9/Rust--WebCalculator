use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};


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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(subtract)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

