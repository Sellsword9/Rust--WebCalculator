

/**
 * This is a simple calculator web app that uses actix-web
 *  to serve the web app and perform the calculations.
 */

// -------------- Crates --------------
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder}; 
use actix_files::Files;

    
/* ENDPOINTS START */
/**
 * (Deprecated)
 * Substracts two numbers both ways
 */
#[get("/subtract/{a}/{b}")] 
async fn subtract(path: web::Path<(i32, i32)>) -> impl Responder { 
    let (a, b) = path.into_inner();
    let result1: i32 = a - b;
    let result2: i32 = b - a;
    HttpResponse::Ok().body(format!("n1 - n2 = {} AND n2 - n1 = {}", result1, result2))
}
/**
 * Calculates the difference between two numbers (Same as subtract)
 */
#[get("/diff/{a}/{b}")] 
async fn diff(path: web::Path<(i32, i32)>) -> impl Responder { 
    let (a, b) = path.into_inner();
    let result: i32 = a - b;
    HttpResponse::Ok().body(format!("±{}", result))
    // The absolute value of the difference IS commutative 
}

#[get("/add/{a}/{b}")]
async fn add(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result: i32 = a + b;
    HttpResponse::Ok().body(result.to_string())
    // Addition is commutative
}
/**
 * Multiplies two numbers
 */
#[get("/multiply/{a}/{b}")]
async fn multiply(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result: i64 = a as i64 * b as i64;
    HttpResponse::Ok().body(result.to_string())
    // Multiplication is commutative!!
}
/**
 * Divides two numbers both ways
 */
#[get("/divide/{a}/{b}")]
async fn divide(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let a: f32 = a as f32;
    let b: f32 = b as f32;
    let result1: f32 = a / b;
    let result2: f32 = b / a;
    HttpResponse::Ok().body(format!("n1 / n2 = {} AND n2 / n1 = {}", result1, result2))
}
/**
 * Raises first number to the power of the second number & the other way around
 */
#[get("/pow/{a}/{b}")]
async fn pow(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let b: f64  = b as f64;
    let a: f64 = a as f64;
    let result1: f64 = a.powf(b);
    let result2: f64 = b.powf(a);
    HttpResponse::Ok().body(format!("n1 ^ n2 = {} AND n2 ^ n1 = {}", result1, result2))
}
/**
 * Returns the n2th root of n1 and the n1th root of n2
 */
#[get("/nrt/{a}/{b}")] 
async fn nrt(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let b: f32 = b as f32;
    let a: f32 = a as f32;
    let result1: f32 = a.powf(1.0/b);
    let result2: f32 = b.powf(1.0/a);
    HttpResponse::Ok().body(format!("n1 ^ (1/n2) = {} AND n2 ^ (1/n1) = {}", result1, result2))
}
/* ENDPOINTS END */

// -------------- Main --------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(subtract)
            .service(diff)
            .service(multiply)
            .service(divide)    
            .service(pow) 
            .service( nrt)
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

