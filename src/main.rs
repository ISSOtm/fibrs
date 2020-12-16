use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use fibrs_lib::caches::SimpleCache;
use std::io;
use fibrs_lib::Cache;

#[get("/status")]
async fn status() -> impl Responder {
	HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // TODO: catch panics in the lib...
	HttpServer::new(|| {
		App::new()
		     .service(status)
	})
    .bind("127.0.0.1:4000")?
    .run()
    .await

    // let mut cache = SimpleCache::new();
    // println!("fib({}) = {}", 60, cache.fib(60));
    // for i in 0..60 {
    //     println!("fib({}) = {}", i, cache.fib(i));
    // }
}
