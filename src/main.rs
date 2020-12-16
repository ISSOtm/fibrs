use fibrs_lib::caches::SimpleCache;
use fibrs_lib::Cache;

fn main() {
    // TODO: catch panics in the lib...

    let mut cache = SimpleCache::new();
    for i in 0..42 {
        println!("fib({}) = {}", i, cache.fib(i));
    }
}
