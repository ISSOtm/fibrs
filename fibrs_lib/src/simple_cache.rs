use crate::Cache;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SimpleCache(Vec<u64>);

impl Cache<usize, u64> for SimpleCache {
    fn new() -> Self {
        let mut cache = Vec::new();
        cache.push(0); // fib(0)
        cache.push(1); // fib(1)
        Self(cache)
    }

    fn fib(&mut self, n: usize) -> u64 {
        // Populate the cache with entries up to the one requested
        for i in self.0.len()..=n {
            let sum = self.0[i - 1] + self.0[i - 2];
            self.0.push(sum);
        }
        self.0[n]
    }

    fn reverse(&self, _: u64) -> usize {
        todo!()
    }
}

// Check the first few in the sequence, which should be a good indication that the algorithm is good
#[test]
fn first_13() {
    let mut cache = SimpleCache::new();
    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    let first_13: Vec<_> = (0..expected.len()).map(|n| cache.fib(n)).collect();
    assert_eq!(first_13, expected);
}
