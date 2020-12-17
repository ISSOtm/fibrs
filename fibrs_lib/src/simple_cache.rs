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

    fn reverse(&mut self, n: u64) -> usize {
        // First, find k such that cache[k-1] < n ≤ cache[k]
        // Does the cache contain such a k?
        let k = if self.0.last().unwrap() < &n {
            // No: populate the cache until `n ≤ cache[k]`
            let mut k = self.0.len() - 1;
            loop {
                let sum = self.0[k - 1] + self.0[k];
                self.0.push(sum);
                k += 1;
                if n <= sum {
                    break;
                }
            }
            k
        } else {
            // Yes: look for the two closest to `n` using dichotomy
            // Invariants: cache[a] < n; n ≤ cache[b]
            let (mut a, mut b) = (0, self.0.len() - 1);
            // Continue until we narrowed down to a single range
            while a != b - 1 {
                let mid = (a + b) / 2;
                if self.0[mid] < n {
                    a = mid;
                } else {
                    b = mid;
                }
            }
            b
        };

        let (ldelta, rdelta) = (n - self.0[k - 1], self.0[k] - n);
        // If ldelta == rdelta, arbitrarily return either
        if ldelta < rdelta {
            k - 1
        } else {
            k
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check the first few in the sequence, which should be a good indication that the algorithm is good
    fn first_13_common<F: FnMut(usize) -> u64>(f: F) {
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        let first_13: Vec<u64> = (0..expected.len()).map(f).collect();
        assert_eq!(first_13, expected);
    }

    // Try computing everything from a single cache...
    #[test]
    fn first_13() {
        let mut cache = SimpleCache::new();
        first_13_common(|n| cache.fib(n));
    }

    // ...and computing each from a fresh cache
    #[test]
    fn first_13_independent() {
        first_13_common(|n| SimpleCache::new().fib(n));
    }

    // Check that `reverse` returns a suitable result for the first ~150 params
    fn rev_first_150_common<F: FnMut(u64) -> usize>(mut f: F) {
        for n in 0..150 {
            // Get `reverse`'s answer
            let k = f(n);

            // Let's not have assertions mutate the cache being tested
            let mut ref_cache = SimpleCache::new();
            // Get the corresponding Fibonacci number
            let closest = ref_cache.fib(k);
            // `n` must be between two Fibonacci numbers, so compute the distance between it and each of them
            let (delta, j, other_delta) = if n < closest {
                (closest - n, k - 1, n - ref_cache.fib(k - 1))
            } else {
                (n - closest, k + 1, ref_cache.fib(k + 1) - n)
            };
            // It's entirely possible that the deltas are identical!
            // e.g. 4 is exactly in the middle of fib(4) = 3, and fib(5) = 5, so both 4 and 5 would be valid
            assert!(
                delta <= other_delta,
                "reverse({}) returned {} (delta {}), but fib({}) = {} only has a delta of {}",
                n,
                k,
                delta,
                j,
                ref_cache.fib(j),
                other_delta
            );
        }
    }

    #[test]
    fn rev_first_150() {
        let mut cache = SimpleCache::new();
        rev_first_150_common(|n| cache.reverse(n));
    }

    #[test]
    fn rev_first_150_independent() {
        rev_first_150_common(|n| SimpleCache::new().reverse(n));
    }
}
