use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use fibrs_lib::caches::SimpleCache;
use std::io;
use std::sync::Mutex;
use fibrs_lib::Cache;

#[derive(Debug)]
struct AppState {
    cache: Mutex<SimpleCache>,
}

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/fib")]
async fn get_fib(data: web::Data<AppState>) -> String {
    // TODO: do not unwrap directly
    let mut cache = data.cache.lock().unwrap();
    // TODO: u64 doesn't work, figure out why
    format!("{}", cache.fib(12))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Data shared by all applications
    let data = web::Data::new(AppState {
        cache: Mutex::new(SimpleCache::new())
    });

    // TODO: catch panics in the lib...
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(status)
            .service(get_fib)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
