mod simple_cache;

pub trait Cache<K, V> {
    fn new() -> Self;

    fn fib(&mut self, n: K) -> V;
    fn reverse(&self, v: V) -> K;
}

pub mod caches {
    pub use crate::simple_cache::SimpleCache;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
