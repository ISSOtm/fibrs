use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use fibrs_lib::Cache;
use std::io;

mod app_state {
    use fibrs_lib::caches::SimpleCache;
    use fibrs_lib::Cache;
    use std::sync::{Mutex, MutexGuard};

    #[derive(Debug)]
    pub struct AppState {
        cache: Mutex<SimpleCache>,
    }

    impl AppState {
        pub fn new() -> Self {
            Self {
                cache: Mutex::new(SimpleCache::new()),
            }
        }
        pub fn get_cache(&self) -> MutexGuard<SimpleCache> {
            // TODO: do not unwrap directly
            // See: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#poisoning
            self.cache.lock().unwrap()
        }
    }
}
use app_state::AppState;

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/fib/{n}")]
async fn get_fib(web::Path(n): web::Path<usize>, app_data: web::Data<AppState>) -> String {
    let k = app_data.get_cache().fib(n);
    format!("{}", k)
}

#[get("/inv/{n}")]
async fn get_rev(web::Path(n): web::Path<u64>, app_data: web::Data<AppState>) -> String {
    let k = app_data.get_cache().reverse(n);
    format!("{}", k)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Data shared by all applications
    let data = web::Data::new(app_state::AppState::new());

    // TODO: catch panics in the lib...
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(status)
            .service(get_fib)
            .service(get_rev)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
