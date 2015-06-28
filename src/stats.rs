// Copyright 2014 Johannes Köster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Mathematical and statistical tools.


use std::cmp;


/// Calculate the number of combinations when choosing
/// k elements from n elements without replacement.
/// This is also known as n over k, or the binomial coefficient.
pub fn comb(n: u64, k: u64) -> f64 {
    if k > n {
        0.0
    }
    else {
        let mut comb = 1.0;
        for j in 0..cmp::min(k, n-k) {
            comb /= (j + 1) as f64;
            comb *= (n - j) as f64;
        }
        comb
    }
}


/// Calculate the number of combinations when choosing
/// k elements from n elements with replacement.
pub fn comb_with_repl(n: u64, k: u64) -> f64 {
    comb(n + k - 1, k)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        assert_eq!(comb(10, 3), 120.0);
        assert_eq!(comb_with_repl(10, 3), 220.0);
        assert_eq!(comb(200, 10), 22451004309013280.0);
    }
}
